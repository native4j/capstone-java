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
 * Operand data for ARM32 instructions.
 */
@JNIClass
@SuppressWarnings("unused")
public class CsOperandArm32 {
    public int vectorIndex;
    public boolean isSubtracted;
    public byte shiftType;
    public long shiftValue;
    public byte operandType;
    public Object rawOperandValue;

    public int getReg() {
        assertType(ARM_OP_REG);
        return (int) rawOperandValue;
    }

    public int getImm() {
        assertType(ARM_OP_IMM);
        return (int) rawOperandValue;
    }

    public CsMemOperandArm64 getMem() {
        assertType(ARM_OP_MEM);
        return (CsMemOperandArm64) rawOperandValue;
    }

    public double getFp() {
        assertType(ARM_OP_FP);
        return (double) rawOperandValue;
    }

    public int getCimm() {
        assertType(ARM_OP_CIMM);
        return (int) rawOperandValue;
    }

    public int getPimm() {
        assertType(ARM_OP_PIMM);
        return (int) rawOperandValue;
    }

    public byte getSetend() {
        assertType(ARM_OP_SETEND);
        return (byte) rawOperandValue;
    }

    public int getSysreg() {
        assertType(ARM_OP_SYSREG);
        return (int) rawOperandValue;
    }

    private void assertType(int type) {
        if (operandType != type) {
            throw new IllegalStateException("Invalid operand type");
        }
    }
}
