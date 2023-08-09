/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone.exception;

/**
 * Generic Capstone exception.
 */
public class CapstoneException extends RuntimeException {
    public CapstoneException(String message) {
        super(message);
    }
}
