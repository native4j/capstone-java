/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone;

import org.native4j.capstone.exception.CapstoneException;
import org.native4j.capstone.insn.CapstoneResult;

/**
 * Wraps a native Capstone instance. This class is thread-safe. <br>
 * {@link #close()} must be called when this instance is no longer needed or
 * memory will be leaked.
 */
@SuppressWarnings("unused")
public class Capstone implements AutoCloseable {
    /**
     * Native handle to the Capstone instance. Do not modify.
     */
    @SuppressWarnings("ALL")
    private long _CsHandle = 0;
    private final CapstoneMode mode;

    /**
     * Initialize a new Capstone instance.
     * 
     * @param mode
     *             The mode to initialize with
     */
    public Capstone(CapstoneMode mode) {
        this.mode = mode;
        check(init(mode));
    }

    /**
     * Close the native Capstone instance.
     */
    @Override
    public void close() {
        check(shutdown());
        assert _CsHandle == 0;
    }

    /**
     * Get the mode this instance was created with.
     * 
     * @return The mode
     */
    public CapstoneMode getMode() {
        return mode;
    }

    /**
     * Disassemble all instructions in the given byte array.
     * 
     * @param result
     *                The result object to populate
     * @param bytes
     *                The bytes to disassemble
     * @param address
     *                The address of the first instruction
     */
    public void disassembleAll(CapstoneResult result, byte[] bytes, long address) {
        check(disassemble(result, bytes, 0, address));
    }

    /**
     * Disassemble instructions, up to the given count.
     * 
     * @param result
     *                The result object to populate
     * @param bytes
     *                The bytes to disassemble
     * @param count
     *                The maximum number of instructions to disassemble
     * @param address
     *                The address of the first instruction
     */
    public void disassembleCount(CapstoneResult result, byte[] bytes, int count, long address) {
        check(disassemble(result, bytes, count, address));
    }

    /**
     * Initializes the native Capstone instance.
     *
     * @param mode
     *             Capstone mode
     * 
     * @return {@code null} if successful, otherwise an error message
     */
    private native String init(CapstoneMode mode);

    /**
     * Shuts down the native Capstone instance.
     *
     * @return {@code null} if successful, otherwise an error message
     */
    private native String shutdown();

    /**
     * See {@link #disassembleCount(CapstoneResult, byte[], int, long)}
     */
    private native String disassemble(CapstoneResult result, byte[] bytes, int count, long address);

    /**
     * Get the name of an instruction.
     * 
     * @param insnId
     *               The instruction id
     * 
     * @return The instruction name or {@code null} if the instruction id is invalid
     */
    public native String getInsnName(int insnId);

    /**
     * Get the name of a register.
     * 
     * @param regId
     *              The register id
     * 
     * @return The register name or {@code null} if the register id is invalid
     */
    public native String getRegName(int regId);

    /**
     * Get the name of an instruction group.
     * 
     * @param groupId
     *                The group id
     *
     * @return The group name or {@code null} if the group id is invalid
     */
    public native String getGroupName(short groupId);

    /**
     * Check the result of a native call and throw an exception if it is not
     * {@code null}.
     * 
     * @param returnValue
     *                    An error message or {@code null} if the call was
     *                    successful
     */
    private static void check(String returnValue) {
        if (returnValue != null) {
            throw new CapstoneException(returnValue);
        }
    }

    static {
        try {
            NativeUtil.loadBindings();
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }
}