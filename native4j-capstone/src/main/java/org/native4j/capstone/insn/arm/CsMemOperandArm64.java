/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone.insn.arm;

import org.native4j.capstone.annotation.JNIClass;

/**
 * ARM64 memory operand.
 */
@JNIClass
@SuppressWarnings("unused")
public class CsMemOperandArm64 {
    public int base;
    public int index;
    public int displacement;

    /* Invoked by JNI */
    public CsMemOperandArm64(int base, int index, int displacement) {
        this.base = base;
        this.index = index;
        this.displacement = displacement;
    }
}
