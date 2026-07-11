use std::collections::{BTreeMap, HashSet};

use zellij_tile::prelude::*;

// Edit this single constant to change the palette. These low-saturation colours
// are deliberately dark enough for common light terminal foregrounds.
const PANE_BACKGROUND_PALETTE: &[&str] = &[
    "#182027", // slate
    "#18212d", // navy slate
    "#17232a", // blue teal
    "#172624", // deep teal
    "#19251f", // forest
    "#20261b", // olive forest
    "#27251b", // dark olive
    "#292219", // brown
    "#2a201d", // umber
    "#2b1e22", // burgundy
    "#291e28", // aubergine
    "#251f2c", // plum
    "#20212e", // indigo slate
    "#1c242b", // cool graphite
    "#202525", // teal graphite
    "#252322", // warm graphite
];

#[derive(Default)]
struct PaneColors {
    handled: HashSet<PaneId>,
    next_palette_index: usize,
    latest_manifest: Option<PaneManifest>,
    subscription_renewed: bool,
}

register_plugin!(PaneColors);

impl ZellijPlugin for PaneColors {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[EventType::PaneUpdate, EventType::PermissionRequestResult]);
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PermissionRequestResult(PermissionStatus::Granted) => {
                // Zellij 0.44 does not emit this event for cached grants. We
                // therefore optimistically handle PaneUpdate below, but replay
                // the latest manifest here for the first-run permission flow,
                // where pre-grant commands were rejected by the host.
                self.handled.clear();
                if let Some(manifest) = self.latest_manifest.clone() {
                    self.handle_manifest(manifest);
                }
            }
            Event::PaneUpdate(manifest) => {
                if !self.subscription_renewed {
                    // With cached permissions, Zellij 0.44.3 can deliver the
                    // initial manifest but omit later PaneUpdate events from
                    // the pre-permission subscription. Renew it once after the
                    // first authorized manifest.
                    subscribe(&[EventType::PaneUpdate, EventType::PermissionRequestResult]);
                    self.subscription_renewed = true;
                }
                self.latest_manifest = Some(manifest.clone());
                self.handle_manifest(manifest);
            }
            _ => {}
        }
        false
    }
}

impl PaneColors {
    fn handle_manifest(&mut self, manifest: PaneManifest) {
        let mut terminals: Vec<_> = manifest
            .panes
            .values()
            .flatten()
            .filter(|pane| !pane.is_plugin)
            .map(|pane| (PaneId::Terminal(pane.id), pane.default_bg.is_some()))
            .collect();

        // Stable ordering makes initial assignment predictable even though the
        // manifest itself is backed by hash maps.
        terminals.sort_by_key(|(pane_id, _)| *pane_id);

        let live_ids: HashSet<_> = terminals.iter().map(|(pane_id, _)| *pane_id).collect();
        self.handled.retain(|pane_id| live_ids.contains(pane_id));

        for (pane_id, has_existing_background) in terminals {
            if self.handled.contains(&pane_id) {
                continue;
            }

            // Mark first: set_pane_color causes another PaneUpdate, and this
            // prevents that event from recolouring the pane.
            self.handled.insert(pane_id);

            // The API exposes the value but not its provenance. Preserve every
            // existing background, including colours explicitly set by layouts.
            if has_existing_background {
                continue;
            }

            let background = PANE_BACKGROUND_PALETTE[self.next_palette_index].to_owned();
            self.next_palette_index = (self.next_palette_index + 1) % PANE_BACKGROUND_PALETTE.len();
            set_pane_color(pane_id, None, Some(background));
        }
    }
}
