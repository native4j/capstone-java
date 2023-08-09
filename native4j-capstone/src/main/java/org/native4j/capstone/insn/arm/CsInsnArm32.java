/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone.insn.arm;

import org.native4j.capstone.annotation.JNIClass;

/**
 * Represents an ARM32 instruction.
 */
@JNIClass
@SuppressWarnings("unused")
public class CsInsnArm32 extends CsInsnArm {
    public boolean isUsermode;
    public int vectorSize;
    public byte vectorData;
    public byte cpsMode;
    public byte cpsFlag;
    public byte conditionCodes;
    public boolean updatesFlags;
    public boolean writebackRequired;
    public byte memBarrier;
    public CsOperandArm32[] operands;
}
