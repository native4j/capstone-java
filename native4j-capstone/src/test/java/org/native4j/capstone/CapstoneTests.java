/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone;

import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

import org.native4j.capstone.insn.CapstoneResult;
import org.native4j.capstone.insn.arm.CsInsnArm64;

public class CapstoneTests {
    private static final byte[] code = new byte[] { (byte) 0xFD, 0x7B, (byte) 0xBF, (byte) 0xA9, (byte) 0xFD, 0x03,
            0x00, (byte) 0x91, 0x00, 0x00, 0x00, (byte) 0x90, 0x00, 0x40, 0x1E, (byte) 0x91, (byte) 0xB3, (byte) 0xFF,
            (byte) 0xFF, (byte) 0x97, 0x00, 0x00, (byte) 0x80, 0x52, (byte) 0xFD, 0x7B, (byte) 0xC1, (byte) 0xA8,
            (byte) 0xC0, 0x03, 0x5F, (byte) 0xD6, };

    private void verifyInstructions(CsInsnArm64[] insns) {
        assertEquals(insns[0].mnemonic, "stp");
        assertEquals(insns[0].operand, "x29, x30, [sp, #-0x10]!");
        assertEquals(insns[0].address, 0x1000);
        assertEquals(insns[0].conditionCodes, 0);

        assertTrue(insns[0].writebackRequired);
        assertFalse(insns[0].updatesFlags);
        assertNull(insns[0].regsRead);
        assertNull(insns[0].regsWrite);
        assertNull(insns[0].groups);

        assertEquals(insns[0].operands[0].getReg(), 2);
        assertEquals(insns[0].operands[1].getReg(), 3);
        assertEquals(insns[0].operands[2].getMem().displacement, -0x10);
    }

    @Test
    void testARM64Disassembly() {
        try (Capstone capstone = new Capstone(CapstoneMode.ARM64)) {
            CapstoneResult result = new CapstoneResult();
            capstone.disassembleAll(result, code, 0x1000);
            CsInsnArm64[] insns = result.toArray(CsInsnArm64[].class);
            verifyInstructions(insns);
        }
    }

    @Test
    void testThreading() {
        try (Capstone capstone = new Capstone(CapstoneMode.ARM64)) {
            List<Thread> threads = new ArrayList<>();
            for (int i = 0; i < 4; i++) {
                Thread th = new Thread(() -> {
                    for (int j = 0; j < 1000; j++) {
                        CapstoneResult result = new CapstoneResult();
                        capstone.disassembleAll(result, code, 0x1000);
                        CsInsnArm64[] insns = result.toArray(CsInsnArm64[].class);
                        verifyInstructions(insns);
                    }
                });
                th.start();
                threads.add(th);
            }
            for (Thread th : threads) {
                try {
                    th.join();
                } catch (InterruptedException e) {
                    fail(e);
                }
            }
        }
    }
}
