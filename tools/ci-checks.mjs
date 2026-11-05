// Each child must succeed; PowerShell's last-command exit status is insufficient.
import {spawnSync} from 'node:child_process';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createLogger} from './script-console.mjs';
export const phases={
  "quality": [
    ["node", "interfaces/controller-wire/consumer.mjs", "--pakar", "--check"],
    ["node", "tests/opcode_registry_test.mjs"],
    ["python", "-B", "tests/mqtt_provisioning_test.py"],
    ["pwsh", "-NoProfile", "-File", "tools/test-cloud-services.ps1"],
    ["pwsh", "-NoProfile", "-File", "tools/test-tls-memory.ps1"],
    ["pwsh", "-NoProfile", "-File", "tools/test-peer-network-commission.ps1"],
    ["node", "tests/resource_report_test.mjs"],
    ["node", "tests/doc_links_test.mjs"],
    ["node", "tools/check-doc-links.mjs"],
    ["pwsh", "-File", "tools/test-user-files.ps1"],
    ["pwsh", "-NoProfile", "-File", "tools/test-ssh.ps1"],
    ["node", "tests/native_dependency_graph_test.mjs"],
    // Static source certificates must fail before expensive SDK/native builds.
    ["node", "tests/pc_contract_mapping_test.mjs"],
    ["pwsh", "-File", "tools/test-radio.ps1"],
    ["pwsh", "-File", "tools/test-peer-identity.ps1"],
    ["pwsh", "-NoProfile", "-File", "tools/test-peer-route.ps1"],
    ["pwsh", "-File", "tests/tool_peer_identity_test.ps1"],
    ["node", "tests/web_peer_version_test.mjs"],
    ["pwsh", "-File", "tools/test-bluetooth.ps1"],
    ["pwsh", "-File", "tools/test-client-management.ps1", "-Restore"],
    ["pwsh", "-File", "tools/test-audio.ps1"],
    ["pwsh", "-File", "tools/test-diagnostic-text.ps1"],
    ["pwsh", "-File", "tools/consult-reuse.ps1", "-Concern", "pu-client-download", "-Gate"],
    ["pwsh", "-File", "tools/consult-reuse.ps1", "-Concern", "pu-cpp-transfer", "-Gate"],
    ["node", "tests/download_firmware_test.mjs"],
    ["pwsh", "-File", "tools/consult-reuse.ps1", "-Concern", "pu-backup-update", "-Gate"],
    ["node", "tests/update_backup_test.mjs"],
    ["node", "tests/verify_running_firmware_test.mjs"],
    ["pwsh", "-File", "tools/consult-reuse.ps1", "-Concern", "pu-client-manifest", "-Gate"],
    ["node", "tests/firmware_manifest_test.mjs"],
    ["node", "tests/script_tooling_test.mjs"],
    ["pwsh", "-File", "tests/script_tooling_test.ps1"],
    ["pwsh", "-File", "tools/consult-reuse.ps1", "-Concern", "pu-tooling", "-Gate"],
    ["pwsh", "-File", "tools/test-update.ps1"],
    ["pwsh", "-File", "tools/test-lcd.ps1"],
    ["node", "tests/lcd_web_test.mjs"],
    ["pwsh", "-File", "tools/consult-reuse.ps1", "-Concern", "ethernet", "-Gate"],
    ["node", "tests/ethernet_live_tool_test.mjs"],
    ["pwsh", "-NoProfile", "-File", "tools/test-ethernet-management.ps1"],
    ["node", "tests/pixel_workbench_live_tool_test.mjs"],
    ["node", "tests/board_silent_test.mjs"],
    ["node", "tests/avr_boot_image_test.mjs"],
    ["node", "tests/windows_tray_test.mjs"],
    ["node", "tests/web_compressed_test.mjs"],
    ["node", "tools/build-web.mjs", "--profile=onboard", "--check"],
    ["pwsh", "-NoProfile", "-File", "tools/test-shell-properties.ps1"],
    ["node", "tests/shell_metadata_test.mjs"],
    ["pwsh", "-NoProfile", "-File", "tests/windows_startup_test.ps1"],
    ["node", "tests/tasks_wiring_test.mjs"],
    ["pwsh", "-File", "tests/device_discovery_script_test.ps1"],
    ["node", "tests/s3_pin_mapping_test.mjs"],
    ["node", "tests/s3_platform_wiring_test.mjs"],
    ["pwsh", "-File", "tools/test-led-pipeline.ps1"],
    ["pwsh", "-File", "tools/test-matrix-fire.ps1"],
    ["node", "tests/esp32_ws2811_reuse_test.mjs"],
    ["pwsh", "-File", "tools/test-led-rmt-ring.ps1"],
    ["node", "tests/web_workspace_test.mjs"],
    ["node", "tests/web_workspace_routes_test.mjs"],
    ["node", "tests/web_dialog_test.mjs"],
    ["node", "tests/web_feature_workspaces_test.mjs"],
    ["node", "tests/web_correction_preview_test.mjs"],
    ["node", "tests/web_appearance_mount_test.mjs"],
    ["node", "tests/web_state_presentation_test.mjs"],
    ["pwsh", "-File", "tools/test-host-presentation.ps1"],
    ["pwsh", "-File", "tools/test-snake.ps1"],
    ["node", "tests/web_cell_assignments_test.mjs"],
    ["pwsh", "-NoProfile", "-File", "tools/test-game-role.ps1"],
    ["pwsh", "-NoProfile", "-File", "tools/test-key-minigames.ps1"],
    ["node", "--test", "tests/key_minigame_integration_test.mjs"],
    ["node", "--test", "tests/web_minigame_test.mjs"],
    ["node", "tests/esp_front_panel_wiring_test.mjs"],
    ["pwsh", "-NoProfile", "-File", "tests/avr_rx_script_test.ps1"],
    ["pwsh", "-File", "tools/test-network.ps1"],
    ["pwsh", "-File", "tools/test-filesystem-telemetry.ps1"],
    ["node", "tests/loop_stack_test.mjs"],
    ["pwsh", "-File", "tools/test-upnp.ps1"],
    ["node", "tests/web_streaming_test.mjs"],
    ["pwsh", "-File", "tools/consult-reuse.ps1", "-Concern", "web-streaming", "-Gate"],
    ["node", "tests/web_peer_http_test.mjs"],
    ["node", "tools/generate-basic-ui.mjs", "--check"],
    ["node", "tests/basic_cells_test.mjs"],
    ["node", "tests/led_api_benchmark_test.mjs"],
    ["node", "tests/peer_fence_test.mjs"],
    ["pwsh", "-File", "tools/consult-reuse.ps1", "-Concern", "peer-binding", "-Gate"],
    ["node", "tests/native_dropdown_test.mjs"],
    ["node", "tests/github_conversation_audit_test.mjs"],
    ["pwsh", "-File", "tools/consult-reuse.ps1", "-Concern", "terminal-ui", "-Gate"],
    ["pwsh", "-File", "tools/consult-reuse.ps1", "-Concern", "api-contract", "-Gate"],
    ["node", "tools/check-living-contract.mjs"],
    ["node", "tests/living_contract_test.mjs"],
    ["pwsh", "-File", "tools/consult-reuse.ps1", "-Concern", "web-presentation", "-Gate"],
    ["node", "tools/generate-font-assets.mjs", "--check"],
    ["node", "tests/font_gzip_test.mjs"],
    ["node", "tests/browser_console_test.mjs"],
    ["node", "tests/toasts_test.mjs"],
    ["node", "tests/realtime_retry_test.mjs"],
    ["node", "tests/build_script_security_test.mjs"],
    ["node", "tests/native_render_lifecycle_test.mjs"],
    ["node", "tests/localization_test.mjs"],
    ["node", "tests/rayanlamp_reuse_test.mjs"],
    ["node", "tests/rayanlamp_theme_test.mjs"],
    ["node", "tests/rayanlamp_locale_test.mjs"],
    [
      "node",
      "tests/web_display_test.mjs"
    ],
    [
      "node",
      "tools/generate-web-appearance.mjs",
      "--check"
    ],
    [
      "node",
      "tests/web_appearance_test.mjs"
    ],
    [
      "node",
      "tests/web_cell_table_test.mjs"
    ],
    [
      "node",
      "tests/web_context_test.mjs"
    ],
    [
      "node",
      "tests/web_identity_test.mjs"
    ],
    [
      "node",
      "tests/screenshot_harness_test.mjs"
    ],
    [
      "node",
      "tests/native_menu_test.mjs"
    ],
    [
      "node",
      "tests/connection_controls_test.mjs"
    ],
    [
      "node",
      "tools/generate-interface-contract.mjs",
      "--check"
    ],
    [
      "node",
      "tests/interface_settings_test.mjs"
    ],
    [
      "node",
      "tools/generate-branding.mjs",
      "--check"
    ],
    [
      "node",
      "tests/branding_test.mjs"
    ],
    [
      "node",
      "tests/web_access_test.mjs"
    ],
    [
      "node",
      "tests/web_remote_test.mjs"
    ],
    [
      "node",
      "tests/remote_tools_test.mjs"
    ],
    [
      "node",
      "tests/pu_routes_test.mjs"
    ],
    [
      "node",
      "tests/boot_audio_test.mjs"
    ],
    [
      "pwsh",
      "-File",
      "tools/check-reuse-ledger.ps1"
    ],
    [
      "pwsh",
      "-File",
      "tests/reuse_ledger_test.ps1"
    ],
    [
      "pwsh",
      "-File",
      "tools/generate-file-comparison.ps1",
      "-Check"
    ],
    [
      "pwsh",
      "-File",
      "tests/file_comparison_test.ps1"
    ],
    [
      "pwsh",
      "-File",
      "tools/verify-versions.ps1"
    ],
    [
      "pwsh",
      "-File",
      "tests/reuse_preflight_test.ps1"
    ],
    [
      "pwsh",
      "-File",
      "tests/build_storage_test.ps1"
    ],
    [
      "pwsh",
      "-File",
      "tests/storage_hygiene_test.ps1"
    ],
    [
      "node",
      "tools/check-crate-boundary.mjs"
    ],
    [
      "node",
      "tests/crate_boundary_test.mjs"
    ],
    [
      "node",
      "tools/generate-readme-artwork.mjs",
      "--check"
    ],
    [
      "node",
      "tests/repository_governance_test.mjs"
    ]
  ],
  "desktop": [
    [
      "pwsh",
      "-File",
      "tools/build-desktop.ps1",
      "-Mode",
      "Test"
    ],
    [
      "pwsh",
      "-File",
      "tools/build-desktop.ps1",
      "-Mode",
      "Release"
    ],
    ["pwsh", "-NoProfile", "-File", "tests/windows_startup_test.ps1", "-Executable", "desktop/target/release/pakarcontroller.exe"],
    [
      "node",
      "tests/terminal_ui_test.mjs"
    ],
    [
      "node",
      "tests/device_cli_test.mjs"
    ],
    ["node", "tests/access_cli_test.mjs"],
    [
      "node",
      "tests/pc_contract_cli_test.mjs"
    ],
    [
      "node",
      "tests/remote_cli_test.mjs"
    ]
  ]
};
export function runChecks(commands,execute=spawnSync){
for(const [cmd,...args] of commands){
  if(args.some(arg=>arg.endsWith('consult-reuse.ps1'))&&args.includes('-Gate')&&!args.includes('-Summary'))args.push('-Summary');
  createLogger().step(cmd+' '+args.join(' '));
  const r=execute(cmd,args,{stdio:'inherit',shell:false});
  if(r.error)throw r.error;
  if(r.status!==0){const error=Error('Required check failed: '+cmd+' '+args.join(' '));error.exitCode=r.status??1;throw error;}
}
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  const phase=process.argv[2];
  if(!Object.hasOwn(phases,phase))throw Error('Usage: node tools/ci-checks.mjs quality|desktop');
  try{runChecks(phases[phase]);}catch(error){console.error(error);process.exit(error.exitCode??1);}
}
