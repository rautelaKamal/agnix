import * as assert from 'assert';
import * as vscode from 'vscode';

suite('Extension Test Suite', () => {
  vscode.window.showInformationMessage('Start all tests.');

  test('Extension should be present', () => {
    assert.ok(vscode.extensions.getExtension('avifenesh.agnix'));
  });

  test('Extension should activate', async () => {
    const ext = vscode.extensions.getExtension('avifenesh.agnix');
    assert.ok(ext);
    await ext.activate();
    assert.ok(ext.isActive);
  });

  suite('Commands', () => {
    test('agnix.restart command should be registered', async () => {
      const commands = await vscode.commands.getCommands(true);
      assert.ok(commands.includes('agnix.restart'));
    });

    test('agnix.showOutput command should be registered', async () => {
      const commands = await vscode.commands.getCommands(true);
      assert.ok(commands.includes('agnix.showOutput'));
    });

    test('agnix.validateFile command should be registered', async () => {
      const commands = await vscode.commands.getCommands(true);
      assert.ok(commands.includes('agnix.validateFile'));
    });

    test('agnix.validateWorkspace command should be registered', async () => {
      const commands = await vscode.commands.getCommands(true);
      assert.ok(commands.includes('agnix.validateWorkspace'));
    });

    test('agnix.showRules command should be registered', async () => {
      const commands = await vscode.commands.getCommands(true);
      assert.ok(commands.includes('agnix.showRules'));
    });

    test('agnix.fixAll command should be registered', async () => {
      const commands = await vscode.commands.getCommands(true);
      assert.ok(commands.includes('agnix.fixAll'));
    });
  });

  suite('Configuration', () => {
    test('agnix.lspPath should have default value', () => {
      const config = vscode.workspace.getConfiguration('agnix');
      const lspPath = config.get<string>('lspPath');
      assert.strictEqual(lspPath, 'agnix-lsp');
    });

    test('agnix.enable should default to true', () => {
      const config = vscode.workspace.getConfiguration('agnix');
      const enable = config.get<boolean>('enable');
      assert.strictEqual(enable, true);
    });

    test('agnix.trace.server should default to off', () => {
      const config = vscode.workspace.getConfiguration('agnix');
      const trace = config.get<string>('trace.server');
      assert.strictEqual(trace, 'off');
    });
  });

  suite('Language Support', () => {
    test('skill-markdown language should be registered', async () => {
      const languages = await vscode.languages.getLanguages();
      assert.ok(languages.includes('skill-markdown'));
    });
  });
});
