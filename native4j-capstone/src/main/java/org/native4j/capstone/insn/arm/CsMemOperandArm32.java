/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone.insn.arm;

import org.native4j.capstone.annotation.JNIClass;

/**
 * ARM32 memory operand.
 */
@JNIClass
@SuppressWarnings("unused")
public class CsMemOperandArm32 {
    public int base;
    public int index;
    public int scale;
    public int displacement;

    /* Invoked by JNI */
    public CsMemOperandArm32(int base, int index, int scale, int displacement) {
        this.base = base;
        this.index = index;
        this.scale = scale;
        this.displacement = displacement;
    }
}
