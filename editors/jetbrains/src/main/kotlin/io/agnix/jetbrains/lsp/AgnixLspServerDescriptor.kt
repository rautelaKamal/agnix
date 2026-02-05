package io.agnix.jetbrains.lsp

import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.project.Project
import com.redhat.devtools.lsp4ij.server.ProcessStreamConnectionProvider
import io.agnix.jetbrains.binary.AgnixBinaryDownloader
import io.agnix.jetbrains.binary.AgnixBinaryResolver
import io.agnix.jetbrains.notifications.AgnixNotifications
import io.agnix.jetbrains.settings.AgnixSettings
import java.io.File
import java.io.InputStream
import java.io.OutputStream

/**
 * LSP server descriptor for agnix.
 *
 * Manages the lifecycle of the agnix-lsp process using stdio transport.
 * Handles binary resolution, download if needed, and process startup.
 */
class AgnixLspServerDescriptor(
    private val project: Project
) : ProcessStreamConnectionProvider() {

    private val logger = Logger.getInstance(AgnixLspServerDescriptor::class.java)
    private var process: Process? = null

    init {
        val binaryPath = resolveBinaryPath()
        if (binaryPath != null) {
            val commands = mutableListOf(binaryPath)
            setCommands(commands)
            setWorkingDirectory(project.basePath ?: System.getProperty("user.home"))
        }
    }

    /**
     * Resolve the path to the agnix-lsp binary.
     *
     * Checks in order:
     * 1. User-configured path in settings
     * 2. Previously downloaded binary in plugin storage
     * 3. System PATH
     */
    private fun resolveBinaryPath(): String? {
        val settings = AgnixSettings.getInstance()

        // Check user-configured path first
        val configuredPath = settings.lspPath
        if (configuredPath.isNotBlank()) {
            val file = File(configuredPath)
            if (file.exists() && file.canExecute()) {
                logger.info("Using configured LSP path: $configuredPath")
                return configuredPath
            }
        }

        // Check for downloaded binary
        val resolver = AgnixBinaryResolver()
        val downloadedPath = resolver.getDownloadedBinaryPath()
        if (downloadedPath != null) {
            logger.info("Using downloaded LSP binary: $downloadedPath")
            return downloadedPath
        }

        // Check system PATH
        val systemPath = resolver.findInPath()
        if (systemPath != null) {
            logger.info("Using system PATH LSP binary: $systemPath")
            return systemPath
        }

        // Binary not found - trigger download
        logger.warn("agnix-lsp binary not found, triggering download")
        AgnixNotifications.notifyBinaryNotFound(project)

        // Attempt download synchronously for first start
        val downloader = AgnixBinaryDownloader()
        val downloaded = downloader.downloadSync()
        if (downloaded != null) {
            logger.info("Downloaded LSP binary to: $downloaded")
            return downloaded
        }

        logger.error("Failed to find or download agnix-lsp binary")
        return null
    }

    override fun start() {
        val commands = getCommands()
        if (commands.isEmpty()) {
            logger.error("No LSP command configured")
            return
        }

        val binaryPath = commands[0]
        if (!File(binaryPath).exists()) {
            logger.error("LSP binary not found: $binaryPath")
            AgnixNotifications.notifyBinaryNotFound(project)
            return
        }

        logger.info("Starting agnix-lsp: ${commands.joinToString(" ")}")
        super.start()
    }

    override fun stop() {
        logger.info("Stopping agnix-lsp")
        super.stop()
    }

    override fun isAlive(): Boolean {
        return super.isAlive()
    }
}
