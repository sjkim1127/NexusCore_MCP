/**
 * Universal Auto-Deobfuscation Script
 * 
 * Detects deobfuscation without rules or hardcoded patterns.
 * Uses entropy analysis and memory tracking.
 * 
 * Detection Methods:
 * 1. Entropy Delta - high entropy → low entropy = decryption
 * 2. Memory Write Tracking - new allocations with readable content
 * 3. Crypto API Interception - results of decryption functions
 */

(function () {
    'use strict';

    var collected = [];
    var ENTROPY_THRESHOLD = 4.5;  // Below this = likely plaintext
    var MIN_STRING_LEN = 6;
    var MAX_STRING_LEN = 2000;

    // Calculate Shannon entropy of byte array
    function calculateEntropy(bytes) {
        if (!bytes || bytes.length === 0) return 0;

        var freq = {};
        var len = bytes.length;

        for (var i = 0; i < len; i++) {
            var b = bytes[i];
            freq[b] = (freq[b] || 0) + 1;
        }

        var entropy = 0;
        for (var key in freq) {
            var p = freq[key] / len;
            entropy -= p * Math.log2(p);
        }

        return entropy;
    }

    // Extract readable strings from buffer
    function extractStrings(ptr, len) {
        var strings = [];
        var current = '';

        try {
            for (var i = 0; i < len && i < MAX_STRING_LEN; i++) {
                var b = ptr.add(i).readU8();
                if (b >= 32 && b <= 126) {
                    current += String.fromCharCode(b);
                } else {
                    if (current.length >= MIN_STRING_LEN) {
                        strings.push(current);
                    }
                    current = '';
                }
            }
            if (current.length >= MIN_STRING_LEN) {
                strings.push(current);
            }
        } catch (e) { }

        return strings;
    }

    // Track VirtualAlloc for new memory regions
    var VirtualAlloc = Module.findExportByName('kernel32.dll', 'VirtualAlloc');
    if (VirtualAlloc) {
        Interceptor.attach(VirtualAlloc, {
            onEnter: function (args) {
                this.size = args[1].toInt32();
            },
            onLeave: function (retval) {
                if (!retval.isNull() && this.size > 0 && this.size < 0x100000) {
                    // Monitor this region for future writes
                    var base = retval;
                    var size = this.size;

                    // Schedule check after some execution
                    setTimeout(function () {
                        try {
                            var data = base.readByteArray(Math.min(size, 4096));
                            if (data) {
                                var arr = new Uint8Array(data);
                                var entropy = calculateEntropy(arr);

                                if (entropy < ENTROPY_THRESHOLD && entropy > 0.5) {
                                    var strings = extractStrings(base, Math.min(size, 4096));
                                    if (strings.length > 0) {
                                        send({
                                            type: 'deobfuscated_region',
                                            address: base.toString(),
                                            entropy: entropy.toFixed(2),
                                            strings: strings.slice(0, 20)
                                        });
                                    }
                                }
                            }
                        } catch (e) { }
                    }, 100);
                }
            }
        });
    }

    // Hook common crypto/decryption functions
    var cryptFuncs = [
        ['advapi32.dll', 'CryptDecrypt'],
        ['bcrypt.dll', 'BCryptDecrypt'],
        ['ncrypt.dll', 'NCryptDecrypt']
    ];

    cryptFuncs.forEach(function (pair) {
        var fn = Module.findExportByName(pair[0], pair[1]);
        if (fn) {
            Interceptor.attach(fn, {
                onEnter: function (args) {
                    // Store buffer pointer for checking after decryption
                    this.pbData = args[3];  // Common position for data buffer
                    this.pdwDataLen = args[4];
                },
                onLeave: function (retval) {
                    if (retval.toInt32() === 1 || retval.toInt32() === 0) {
                        try {
                            var len = this.pdwDataLen.readU32();
                            if (len > 0 && len < 10000) {
                                var strings = extractStrings(this.pbData, len);
                                if (strings.length > 0) {
                                    send({
                                        type: 'crypto_decrypted',
                                        api: pair[1],
                                        length: len,
                                        strings: strings.slice(0, 10)
                                    });
                                }
                            }
                        } catch (e) { }
                    }
                }
            });
        }
    });

    // Hook RtlDecompressBuffer for unpacking
    var RtlDecompress = Module.findExportByName('ntdll.dll', 'RtlDecompressBuffer');
    if (RtlDecompress) {
        Interceptor.attach(RtlDecompress, {
            onEnter: function (args) {
                this.outBuf = args[1];
                this.outSize = args[2].toInt32();
            },
            onLeave: function (retval) {
                if (retval.toInt32() === 0) {  // STATUS_SUCCESS
                    try {
                        var strings = extractStrings(this.outBuf, Math.min(this.outSize, 4096));
                        if (strings.length > 0) {
                            send({
                                type: 'decompressed',
                                strings: strings.slice(0, 20)
                            });
                        }
                    } catch (e) { }
                }
            }
        });
    }

    send({
        type: 'deobfuscator_active',
        methods: ['entropy_analysis', 'crypto_hooks', 'decompression_hooks', 'memory_tracking']
    });

})();
