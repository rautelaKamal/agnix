package io.agnix.jetbrains.binary

import com.intellij.openapi.application.PathManager
import com.intellij.openapi.diagnostic.Logger
import java.io.File
import java.nio.file.Paths

/**
 * Resolves the location of the agnix-lsp binary.
 *
 * Searches for the binary in multiple locations:
 * 1. Plugin storage directory (downloaded binary)
 * 2. System PATH
 * 3. Common installation directories
 */
class AgnixBinaryResolver {

    private val logger = Logger.getInstance(AgnixBinaryResolver::class.java)

    companion object {
        const val BINARY_NAME = "agnix-lsp"
        const val BINARY_NAME_WINDOWS = "agnix-lsp.exe"
    }

    /**
     * Get the plugin storage directory for downloaded binaries.
     */
    fun getStorageDirectory(): File {
        val pluginDir = PathManager.getPluginsPath()
        return File(pluginDir, "agnix/bin")
    }

    /**
     * Get the path to the downloaded binary, if it exists.
     */
    fun getDownloadedBinaryPath(): String? {
        val binaryInfo = PlatformInfo.getBinaryInfo() ?: return null
        val binaryFile = File(getStorageDirectory(), binaryInfo.binaryName)

        return if (binaryFile.exists() && binaryFile.canExecute()) {
            binaryFile.absolutePath
        } else {
            null
        }
    }

    /**
     * Find agnix-lsp in the system PATH.
     */
    fun findInPath(): String? {
        val binaryName = getBinaryName()
        val pathEnv = System.getenv("PATH") ?: return null
        val pathSeparator = File.pathSeparator
        val extensions = if (PlatformInfo.getOS() == PlatformInfo.OS.WINDOWS) {
            listOf("", ".exe", ".cmd", ".bat")
        } else {
            listOf("")
        }

        for (dir in pathEnv.split(pathSeparator)) {
            for (ext in extensions) {
                val file = File(dir, binaryName + ext)
                if (file.exists() && file.canExecute()) {
                    logger.info("Found agnix-lsp in PATH: ${file.absolutePath}")
                    return file.absolutePath
                }
            }
        }

        return null
    }

    /**
     * Find agnix-lsp in common installation directories.
     */
    fun findInCommonLocations(): String? {
        val binaryName = getBinaryName()
        val homeDir = System.getProperty("user.home")

        val locations = when (PlatformInfo.getOS()) {
            PlatformInfo.OS.MACOS -> listOf(
                "/usr/local/bin/$binaryName",
                "/opt/homebrew/bin/$binaryName",
                "$homeDir/.cargo/bin/$binaryName",
                "$homeDir/.local/bin/$binaryName"
            )
            PlatformInfo.OS.LINUX -> listOf(
                "/usr/local/bin/$binaryName",
                "/usr/bin/$binaryName",
                "$homeDir/.cargo/bin/$binaryName",
                "$homeDir/.local/bin/$binaryName"
            )
            PlatformInfo.OS.WINDOWS -> listOf(
                "$homeDir\\.cargo\\bin\\$binaryName",
                "C:\\Program Files\\agnix\\$binaryName"
            )
            PlatformInfo.OS.UNKNOWN -> emptyList()
        }

        for (location in locations) {
            val file = File(location)
            if (file.exists() && file.canExecute()) {
                logger.info("Found agnix-lsp at: ${file.absolutePath}")
                return file.absolutePath
            }
        }

        return null
    }

    /**
     * Resolve the binary path using all available methods.
     *
     * Priority:
     * 1. Downloaded binary in plugin storage
     * 2. System PATH
     * 3. Common installation locations
     */
    fun resolve(): String? {
        // Check downloaded binary
        getDownloadedBinaryPath()?.let { return it }

        // Check PATH
        findInPath()?.let { return it }

        // Check common locations
        findInCommonLocations()?.let { return it }

        logger.warn("agnix-lsp binary not found")
        return null
    }

    /**
     * Get the expected binary name for the current platform.
     */
    private fun getBinaryName(): String {
        return if (PlatformInfo.getOS() == PlatformInfo.OS.WINDOWS) {
            BINARY_NAME_WINDOWS
        } else {
            BINARY_NAME
        }
    }

    /**
     * Check if a binary exists at the given path and is executable.
     */
    fun isValidBinary(path: String): Boolean {
        val file = File(path)
        return file.exists() && file.canExecute()
    }
}
