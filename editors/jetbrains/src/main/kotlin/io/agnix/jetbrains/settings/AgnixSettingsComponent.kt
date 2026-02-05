package io.agnix.jetbrains.settings

import com.intellij.openapi.fileChooser.FileChooserDescriptorFactory
import com.intellij.openapi.ui.TextFieldWithBrowseButton
import com.intellij.ui.components.JBCheckBox
import com.intellij.ui.components.JBLabel
import com.intellij.util.ui.FormBuilder
import javax.swing.JComboBox
import javax.swing.JComponent
import javax.swing.JPanel

/**
 * UI component for agnix settings.
 *
 * Provides form fields for configuring the plugin.
 */
class AgnixSettingsComponent {

    private val mainPanel: JPanel
    private val enabledCheckBox = JBCheckBox("Enable agnix validation")
    private val lspPathField = TextFieldWithBrowseButton()
    private val autoDownloadCheckBox = JBCheckBox("Automatically download LSP binary if not found")
    private val traceLevelComboBox = JComboBox(AgnixSettings.TraceLevel.entries.toTypedArray())
    private val codeLensCheckBox = JBCheckBox("Show CodeLens annotations")

    init {
        // Configure file chooser for LSP path
        lspPathField.addBrowseFolderListener(
            "Select agnix-lsp Binary",
            "Choose the path to the agnix-lsp executable",
            null,
            FileChooserDescriptorFactory.createSingleFileDescriptor()
        )

        // Build the form
        mainPanel = FormBuilder.createFormBuilder()
            .addComponent(enabledCheckBox)
            .addSeparator()
            .addLabeledComponent(JBLabel("LSP binary path:"), lspPathField, 1, false)
            .addTooltip("Leave empty to use auto-detection or downloaded binary")
            .addComponent(autoDownloadCheckBox)
            .addSeparator()
            .addLabeledComponent(JBLabel("Trace level:"), traceLevelComboBox, 1, false)
            .addTooltip("Set to 'Messages' or 'Verbose' for debugging LSP communication")
            .addComponent(codeLensCheckBox)
            .addComponentFillVertically(JPanel(), 0)
            .panel
    }

    /**
     * Get the main settings panel.
     */
    fun getPanel(): JComponent = mainPanel

    /**
     * Get the preferred focus component.
     */
    fun getPreferredFocusedComponent(): JComponent = enabledCheckBox

    /**
     * Whether the plugin is enabled.
     */
    var enabled: Boolean
        get() = enabledCheckBox.isSelected
        set(value) {
            enabledCheckBox.isSelected = value
        }

    /**
     * The LSP binary path.
     */
    var lspPath: String
        get() = lspPathField.text
        set(value) {
            lspPathField.text = value
        }

    /**
     * Whether to auto-download the LSP binary.
     */
    var autoDownload: Boolean
        get() = autoDownloadCheckBox.isSelected
        set(value) {
            autoDownloadCheckBox.isSelected = value
        }

    /**
     * The trace level for LSP communication.
     */
    var traceLevel: AgnixSettings.TraceLevel
        get() = traceLevelComboBox.selectedItem as AgnixSettings.TraceLevel
        set(value) {
            traceLevelComboBox.selectedItem = value
        }

    /**
     * Whether CodeLens is enabled.
     */
    var codeLensEnabled: Boolean
        get() = codeLensCheckBox.isSelected
        set(value) {
            codeLensCheckBox.isSelected = value
        }
}
