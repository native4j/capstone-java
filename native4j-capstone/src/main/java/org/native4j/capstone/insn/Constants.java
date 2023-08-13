/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone.insn;

/**
 * Generic Capstone constants.
 */
@SuppressWarnings("ALL")
public class Constants {
    public static final int CS_GRP_INVALID = 0;
    public static final int CS_GRP_JUMP = 1;
    public static final int CS_GRP_CALL = 2;
    public static final int CS_GRP_RET = 3;
    public static final int CS_GRP_INT = 4;
    public static final int CS_GRP_IRET = 5;
    public static final int CS_GRP_PRIVILEGE = 6;
    public static final int CS_GRP_BRANCH_RELATIVE = 7;
}
