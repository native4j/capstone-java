/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone.insn;

import java.util.AbstractList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

import org.jetbrains.annotations.Unmodifiable;

/**
 * Contains the instruction results from disassembly. <br>
 * An instance of this class <b>must not be used between
 * {@link org.native4j.capstone.Capstone} instances!</b>
 */
@SuppressWarnings("unused")
public final class CapstoneResult {
    /* Set directly via JNI. Do not modify. */
    @SuppressWarnings({ "MismatchedReadAndWriteOfArray" })
    private CsInsn[] instructions;
    /* Set directly via JNI */
    private int instructionCount;

    private List<? extends CsInsn> instructionView;

    public CapstoneResult() {
    }

    /**
     * Returns a copy of the instructions array.
     *
     * @param tClass
     *               The class of T
     *
     * @return A copy of the instructions array
     * 
     * @param <T>
     *            The instruction class to cast to
     */
    public <T extends CsInsn> T[] toArray(Class<? extends T[]> tClass) {
        return Arrays.copyOf(instructions, instructionCount, tClass);
    }

    /**
     * Returns an unmodifiable view of the disassembled instructions.
     *
     * @return An unmodifiable view of the disassembled instructions
     *
     * @param <T>
     *            The instruction class to cast to
     */
    @Unmodifiable
    @SuppressWarnings("unchecked")
    public <T extends CsInsn> List<T> getInstructions() {
        if (instructionView == null) {
            instructionView = Collections.unmodifiableList(new AbstractList<>() {
                @Override
                public T get(int index) {
                    return (T) instructions[index];
                }

                @Override
                public int size() {
                    return instructionCount;
                }
            });
        }
        return (List<T>) instructionView;
    }
}
