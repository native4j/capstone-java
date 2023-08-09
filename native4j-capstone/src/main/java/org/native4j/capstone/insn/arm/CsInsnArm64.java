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
public class CsInsnArm64 extends CsInsnArm {
    public byte conditionCodes;
    public boolean updatesFlags;
    public boolean writebackRequired;
    public CsOperandArm64[] operands;
}
