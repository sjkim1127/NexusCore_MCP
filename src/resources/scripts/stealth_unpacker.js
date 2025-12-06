/*
    NexusCore Stealth Unpacker Script
    Bypasses common anti-debugging and identification checks used by Themida/VMProtect.
*/

// 1. Anti-Debug Bypass (Basic)
const kernel32 = Module.findBaseAddress("kernel32.dll");
const ntdll = Module.findBaseAddress("ntdll.dll");

if (kernel32) {
    Interceptor.attach(Module.getExportByName("kernel32.dll", "IsDebuggerPresent"), {
        onLeave: function (retval) {
            // console.log("[*] Intercepted IsDebuggerPresent, returning 0");
            retval.replace(0);
        }
    });

    Interceptor.attach(Module.getExportByName("kernel32.dll", "CheckRemoteDebuggerPresent"), {
        onLeave: function (retval) {
            // CheckRemoteDebuggerPresent(hProcess, &bDebuggerPresent)
            // We need to write 0 to the second argument pointer
            if (this.context.rdx) { // x64 calling convention, 2nd arg in RDX
                ptr(this.context.rdx).writeU32(0);
            }
            // console.log("[*] Intercepted CheckRemoteDebuggerPresent, returning 0");
            retval.replace(0);
        }
    });
}

// 2. NT Level Bypass (Slightly more advanced)
if (ntdll) {
    // NtQueryInformationProcess (ProcessDebugPort = 7, ProcessDebugObjectHandle = 30)
    Interceptor.attach(Module.getExportByName("ntdll.dll", "NtQueryInformationProcess"), {
        onEnter: function (args) {
            this.infoClass = args[1].toInt32();
            this.retPtr = args[3];
        },
        onLeave: function (retval) {
            if (this.infoClass === 7 || this.infoClass === 30) { // DebugPort or DebugObject
                if (!this.retPtr.isNull()) {
                    // console.log("[*] Faking NtQueryInformationProcess (DebugPort/Handle)");
                    this.retPtr.writeU64(0); // Clear the result buffer
                }
                retval.replace(0); // STATUS_SUCCESS
            }
        }
    });

    // NtSetInformationThread (HideFromDebugger = 0x11)
    Interceptor.attach(Module.getExportByName("ntdll.dll", "NtSetInformationThread"), {
        onEnter: function (args) {
            const threadInfoClass = args[1].toInt32();
            if (threadInfoClass === 0x11) {
                // console.log("[*] Prevented NtSetInformationThread(HideFromDebugger)");
                // We can't easily return early in onEnter, but we can corrupt the call or just log.
                // Better approach: detach debugger momentarily or just ignore if we are injected.
                // For now, let's change the info class to something harmless (e.g. 0)
                args[1] = ptr(0);
            }
        }
    });
}

// 3. Generic Unpacking Heuristic (Memory Allocations)
// Watch for VirtualAlloc/VirtualProtect creating RWX memory -> often payload destination
const VirtualAlloc = Module.getExportByName("kernel32.dll", "VirtualAlloc");
Interceptor.attach(VirtualAlloc, {
    onEnter: function (args) {
        this.size = args[1];
        this.protect = args[3].toInt32();
    },
    onLeave: function (retval) {
        // 0x40 = PAGE_EXECUTE_READWRITE
        if (this.protect === 0x40 && !retval.isNull()) {
            send({
                "type": "unpacker_heuristic",
                "event": "RWX_Allocation",
                "address": retval.toString(),
                "size": this.size.toString()
            });
            // console.log(`[*] RWX Block Allocated at ${retval} size ${this.size}`);
        }
    }
});

console.log("[NexusCore] Stealth Unpacker Loaded - Hooks Installed");
