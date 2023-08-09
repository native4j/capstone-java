/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone.insn.arm;

import static org.native4j.capstone.insn.arm.ArmConstants.*;

import org.native4j.capstone.annotation.JNIClass;

/**
 * Operand data for ARM64 instructions.
 */
@JNIClass
@SuppressWarnings("unused")
public class CsOperandArm64 {
    public int vectorIndex;
    public byte vas;
    public byte shiftType;
    public long shiftValue;
    public byte ext;
    public byte operandType;
    public Object rawOperandValue;

    public int getReg() {
        assertType(ARM64_OP_REG);
        return (int) rawOperandValue;
    }

    public long getImm() {
        assertType(ARM64_OP_MEM);
        return (long) rawOperandValue;
    }

    public CsMemOperandArm64 getMem() {
        assertType(ARM64_OP_MEM);
        return (CsMemOperandArm64) rawOperandValue;
    }

    public double getFp() {
        assertType(ARM64_OP_FP);
        return (double) rawOperandValue;
    }

    public long getCimm() {
        assertType(ARM64_OP_CIMM);
        return (long) rawOperandValue;
    }

    public int getRegMrs() {
        assertType(ARM64_OP_REGMRS);
        return getReg();
    }

    public int getRegMsr() {
        assertType(ARM64_OP_REGMSR);
        return getReg();
    }

    public short getPstate() {
        assertType(ARM64_OP_PSTATE);
        return (byte) rawOperandValue;
    }

    public long getSys() {
        assertType(ARM64_OP_SYS);
        return (int) rawOperandValue & 0xffffffffL;
    }

    public byte getPrefetch() {
        assertType(ARM64_OP_PREFETCH);
        return (byte) rawOperandValue;
    }

    public byte getBarrier() {
        assertType(ARM64_OP_BARRIER);
        return (byte) rawOperandValue;
    }

    private void assertType(int type) {
        if (operandType != type) {
            throw new IllegalStateException("Invalid operand type");
        }
    }
}
