import 'package:flutter/material.dart';

import 'api.dart';
import 'app_support.dart';
import 'app_update_coordinator.dart';
import 'app_updates_page.dart';
import 'firmware_automation.dart';
import 'firmware_page.dart';
import 'icons.dart';
import 'presentation_assets.dart';
import 'workspace_widgets.dart';

/// One target-oriented entry: manual actions and automatic policy live together.
class UpdateCenter extends StatefulWidget {
  const UpdateCenter({
    super.key,
    required this.api,
    required this.apps,
    required this.support,
    required this.authorized,
    required this.busyChanged,
    this.automation,
    this.boardAvailable = false,
    this.boardProductionHold = false,
    this.openBoard = false,
  });
  final PakarApi api;
  final AppUpdateCoordinator apps;
  final AppSupportSession support;
  final FirmwareAutomation? automation;
  final bool Function(String) authorized;
  final ValueChanged<bool> busyChanged;
  final bool boardAvailable;
  final bool boardProductionHold;
  final bool openBoard;
  @override
  State<UpdateCenter> createState() => _UpdateCenterState();
}

class _UpdateCenterState extends State<UpdateCenter> {
  bool busy = false;
  @override
  Widget build(BuildContext context) => DefaultTabController(
    length: widget.boardAvailable ? 2 : 1,
    initialIndex: widget.openBoard && widget.boardAvailable ? 1 : 0,
    child: PopScope(
      canPop: !busy,
      child: Scaffold(
        appBar: AppBar(
          title: const Text('Updates & support'),
          actions: [
            if (widget.boardProductionHold)
              const Padding(
                padding: EdgeInsets.symmetric(horizontal: 12),
                child: Tooltip(
                  message: 'Board auto-updates paused for production',
                  child: Icon(PakarIcons.lock),
                ),
              ),
          ],
          automaticallyImplyLeading: !busy,
          bottom: widget.boardAvailable
              ? const TabBar(
                  tabs: [
                    Tab(text: 'Application'),
                    Tab(text: 'ESP / AVR'),
                  ],
                )
              : null,
        ),
        body: TabBarView(
          physics: busy ? const NeverScrollableScrollPhysics() : null,
          children: [
            AppUpdatesPage(
              coordinator: widget.apps,
              extensions: [
                const PresentationUpdatesSection(),
                SupportSection(session: widget.support),
              ],
              // Data-only updates are the lightweight path; APK transfer stays
              // here for compiled Flutter/native changes, not another menu.
              extensionsFirst: true,
            ),
            if (widget.boardAvailable)
              SingleChildScrollView(
                padding: const EdgeInsets.all(16),
                child: FirmwarePage(
                  api: widget.api,
                  automation: widget.automation,
                  authorized: widget.authorized,
                  busyChanged: (value) {
                    widget.busyChanged(value);
                    if (mounted) setState(() => busy = value);
                  },
                ),
              ),
          ],
        ),
      ),
    ),
  );
}

class SupportSection extends StatefulWidget {
  const SupportSection({super.key, required this.session});
  final AppSupportSession session;
  @override
  State<SupportSection> createState() => _SupportSectionState();
}

class _SupportSectionState extends State<SupportSection> {
  bool commands = false, assets = false, working = false, reveal = false;
  String error = '';
  Future<void> change() async {
    setState(() {
      working = true;
      error = '';
    });
    try {
      if (widget.session.running) {
        await widget.session.stop();
      } else {
        await widget.session.start(commands: commands, assets: assets);
      }
    } on Object catch (e) {
      if (mounted) setState(() => error = '$e');
    } finally {
      if (mounted) setState(() => working = false);
    }
  }

  @override
  Widget build(BuildContext context) => AnimatedBuilder(
    animation: widget.session,
    builder: (context, _) => WorkspaceSection(
      title: 'Remote troubleshooting',
      icon: PakarIcons.circuitry,
      children: [
        SwitchListTile.adaptive(
          contentPadding: EdgeInsets.zero,
          title: const Text('Allow board commands in this session'),
          value: commands,
          onChanged: working || widget.session.running
              ? null
              : (v) => setState(() => commands = v),
        ),
        SwitchListTile.adaptive(
          contentPadding: EdgeInsets.zero,
          title: const Text('Allow data-only presentation updates'),
          value: assets,
          onChanged: working || widget.session.running
              ? null
              : (v) => setState(() => assets = v),
        ),
        OutlinedButton.icon(
          onPressed: working ? null : change,
          icon: const Icon(PakarIcons.link),
          label: Text(
            widget.session.running
                ? 'End support session'
                : 'Start support session',
          ),
        ),
        if (widget.session.running) ...[
          const SizedBox(height: 12),
          SelectableText(
            'LAN port ${widget.session.port} · session ${widget.session.session}',
          ),
          const Text(
            'HTTP /api/support/health · /api/support/request · WebSocket /api/support/ws\nhealth · board.status · board.mapping · assets.check · assets.apply',
          ),
          TextButton(
            onPressed: () => setState(() => reveal = !reveal),
            child: Text(reveal ? 'Hide access token' : 'Show access token'),
          ),
          if (reveal) SelectableText(widget.session.token),
          const Text(
            'Foreground only · expires after 15 minutes · Bearer token required',
          ),
        ],
        if (error.isNotEmpty)
          Text(
            error,
            style: TextStyle(color: Theme.of(context).colorScheme.error),
          ),
      ],
    ),
  );
}

class PresentationUpdatesSection extends StatefulWidget {
  const PresentationUpdatesSection({super.key});
  @override
  State<PresentationUpdatesSection> createState() =>
      _PresentationUpdatesSectionState();
}

class _PresentationUpdatesSectionState
    extends State<PresentationUpdatesSection> {
  final source = TextEditingController(text: presentationAssets.source);
  bool enabled = presentationAssets.enabled, working = false;
  String error = '';
  @override
  void dispose() {
    source.dispose();
    super.dispose();
  }

  Future<void> run(bool check) async {
    setState(() {
      working = true;
      error = '';
    });
    try {
      await presentationAssets.configure(source.text, enabled);
      if (check) await presentationAssets.checkNow();
    } on Object catch (e) {
      if (mounted) setState(() => error = '$e');
    } finally {
      if (mounted) setState(() => working = false);
    }
  }

  @override
  Widget build(BuildContext context) => AnimatedBuilder(
    animation: presentationAssets,
    builder: (context, _) => WorkspaceSection(
      title: 'Presentation assets',
      icon: PakarIcons.gridFour,
      children: [
        Text('Revision ${presentationAssets.revision} · no app reinstall'),
        SwitchListTile.adaptive(
          contentPadding: EdgeInsets.zero,
          title: const Text('Keep presentation data up to date'),
          value: enabled,
          onChanged: working ? null : (v) => setState(() => enabled = v),
        ),
        TextField(
          controller: source,
          enabled: !working,
          decoration: const InputDecoration(
            labelText: 'Trusted presentation manifest URL',
          ),
        ),
        const SizedBox(height: 12),
        Wrap(
          spacing: 8,
          runSpacing: 8,
          children: [
            OutlinedButton(
              onPressed: working ? null : () => run(false),
              child: const Text('Save source'),
            ),
            OutlinedButton(
              onPressed: working ? null : () => run(true),
              child: const Text('Check now'),
            ),
          ],
        ),
        if (presentationAssets.busy) const LinearProgressIndicator(),
        if (error.isNotEmpty || presentationAssets.message.isNotEmpty)
          Text(error.isNotEmpty ? error : presentationAssets.message),
      ],
    ),
  );
}
