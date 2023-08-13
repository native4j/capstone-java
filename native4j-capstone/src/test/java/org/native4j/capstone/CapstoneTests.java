/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicBoolean;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

import org.native4j.capstone.exception.CapstoneException;
import org.native4j.capstone.insn.CapstoneResult;
import org.native4j.capstone.insn.arm.CsInsnArm64;

public class CapstoneTests {
    private static final byte[] code = new byte[] { (byte) 0xFD, 0x7B, (byte) 0xBF, (byte) 0xA9, (byte) 0xFD, 0x03,
            0x00, (byte) 0x91, 0x00, 0x00, 0x00, (byte) 0x90, 0x00, 0x40, 0x1E, (byte) 0x91, (byte) 0xB3, (byte) 0xFF,
            (byte) 0xFF, (byte) 0x97, 0x00, 0x00, (byte) 0x80, 0x52, (byte) 0xFD, 0x7B, (byte) 0xC1, (byte) 0xA8,
            (byte) 0xC0, 0x03, 0x5F, (byte) 0xD6, };

    private boolean verifyInstructions(Capstone capstone, CsInsnArm64[] insns) {
        CsInsnArm64 first = insns[0];

        assertEquals(first.mnemonic, "stp");
        assertEquals(capstone.getInsnName(first.instructionId), "stp");

        assertEquals(first.operand, "x29, x30, [sp, #-0x10]!");
        assertEquals(first.address, 0x1000);
        assertEquals(first.conditionCodes, 0);

        assertTrue(first.writebackRequired);
        assertFalse(first.updatesFlags);
        assertNull(first.regsRead);
        assertNull(first.regsWrite);
        assertNull(first.groups);

        assertEquals(first.operands[0].getReg(), 2);
        assertEquals(first.operands[1].getReg(), 3);
        assertEquals(first.operands[2].getMem().displacement, -0x10);

        CsInsnArm64 fifth = insns[4];
        assertEquals(fifth.mnemonic, "bl");
        assertEquals(capstone.getInsnName(fifth.instructionId), "bl");

        assertEquals(fifth.operands[0].getImm(), 3804);

        assertEquals(capstone.getGroupName(fifth.groups[0]), "call");
        assertEquals(capstone.getGroupName(fifth.groups[1]), "jump");
        assertEquals(capstone.getGroupName(fifth.groups[2]), "branch_relative");

        return true;
    }

    @Test
    void testARM64Disassembly() {
        try (Capstone capstone = new Capstone(CapstoneMode.ARM64)) {
            CapstoneResult result = new CapstoneResult();
            capstone.disassembleAll(result, code, 0x1000);
            CsInsnArm64[] insns = result.toArray(CsInsnArm64[].class);
            verifyInstructions(capstone, insns);
        }
    }

    @Test
    void testNullHandle() {
        {
            Capstone capstone = new Capstone(CapstoneMode.ARM64);
            capstone.close();
            assertThrows(CapstoneException.class, () -> capstone.getInsnName(0));
        }
        {
            Capstone capstone = new Capstone(CapstoneMode.ARM64);
            capstone.close();
            assertThrows(CapstoneException.class, capstone::close);
        }
    }

    @Test
    void testThreading() throws InterruptedException {
        AtomicBoolean failed = new AtomicBoolean(true);
        try (Capstone capstone = new Capstone(CapstoneMode.ARM64)) {
            List<Thread> threads = new ArrayList<>();
            for (int i = 0; i < 4; i++) {
                Thread th = new Thread(() -> {
                    for (int j = 0; j < 1000; j++) {
                        CapstoneResult result = new CapstoneResult();
                        capstone.disassembleAll(result, code, 0x1000);
                        CsInsnArm64[] insns = result.toArray(CsInsnArm64[].class);
                        failed.set(!verifyInstructions(capstone, insns));
                    }
                });
                th.start();
                threads.add(th);
            }
            for (Thread th : threads) {
                th.join();
            }
        }
        if (failed.get())
            fail("Failed to verify instructions");
    }
}
