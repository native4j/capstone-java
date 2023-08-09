/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE
 * file.
 */
package org.native4j.capstone;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;

/**
 * Native utility methods.
 */
class NativeUtil {
    /**
     * Load the native Capstone bindings.
     * 
     * @throws IOException
     *                     The bindings could not be loaded
     */
    static void loadBindings() throws IOException {
        String arch = System.getProperty("os.arch");
        String os = System.getProperty("os.name").toLowerCase();

        // "Windows 10" -> "windows"
        // "Mac OS X" -> "mac"
        String[] split = os.split(" ");
        if (split.length > 1)
            os = split[0];

        String libraryName = getLibraryName(os);

        String libraryPath = String.format("%s-%s/%s", os, arch, libraryName);

        Path tempDirectory = Files.createTempDirectory("capstone-java-natives");
        File tempFile = tempDirectory.resolve(libraryName).toFile();

        tempDirectory.toFile().deleteOnExit();

        try (InputStream stream = NativeUtil.class.getClassLoader().getResourceAsStream(libraryPath)) {
            if (stream == null)
                throw new IOException("failed to find Capstone bindings");
            Files.write(tempFile.toPath(), stream.readAllBytes());
        }

        System.load(tempDirectory.resolve(libraryName).toString());
    }

    /**
     * Get the dynamic library name for the current operating system.
     *
     * @param os
     *           The operating system name
     * 
     * @return The dynamic library name
     */
    private static String getLibraryName(String os) {
        if (os.contains("windows"))
            return "capstone_java.dll";
        if (os.contains("nix") || os.contains("nux"))
            return "libcapstone_java.so";
        if (os.contains("mac"))
            return "libcapstone_java.dylib";
        throw new UnsupportedOperationException("unsupported operating system: " + os);
    }
}
