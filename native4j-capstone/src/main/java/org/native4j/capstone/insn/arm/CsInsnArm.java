/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone.insn.arm;

import org.native4j.capstone.annotation.JNIClass;
import org.native4j.capstone.insn.CsInsn;

/**
 * Shared ARM instruction fields.
 */
@JNIClass
@SuppressWarnings("unused")
public abstract class CsInsnArm extends CsInsn {
    public String mnemonic;
    public String operand;
    public int instructionId;
    public int size;
    public long address;
    public short[] regsRead;
    public short[] regsWrite;
    public short[] groups;
}
