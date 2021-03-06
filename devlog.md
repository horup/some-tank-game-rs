--- for v1.2 ---
- 2021-08-06  10m fixed bevy title and subsystem
- 2021-08-05  45m replaced bevy_ui with egui in the hud system, removed console impl as this needs rework
- 2021-08-04  70m worked on egui integration, got text rendering to work, worked on fading using egui painter
- 2021-08-03  90m worked on egui integration by replacing Bevy  UI with egui


--- for v1.1 ---
- 2021-07-22  40m got touch support to work
- 2021-07-21  75m worked on touch support, got something to partially work.
- 2021-07-20  30m worked on touch events in the browser, but no luck since winit does not yet support touch events targeting canvas html5 :-(
- 2021-07-19  55m worked on rendering of waypoints
- 2021-07-18  90m worked on autopilot feature for keyboard less controls
- 2021-07-17  60m made a bit more dpi aware, refactored mouse system a bit, made hud scale factor aware
- 2021-07-16  60m tested wasm in different mobile browsers
- 2021-07-15  15m improved error handling a bit,
- 2021-07-14  60m worked on preloading assets, added spinner to index.html
- 2021-07-13  25m+25m tried to improve web loading
- 2021-07-12  30m worked on ci/cd, got stuff to build for github pages, removed audio hack as it seems to be causing issues
- 2021-07-11  70m game now functions in the browser after embedded the tsx tilesets! 
- 2021-07-10  10m started refactoring to allow web target, bumped version


--- 3905m = 65h for v1 ---
- 2021-07-02  20m updated some stuff related to installer and added credits to splash
- 2021-07-01  30m fixed corrupted setup
- 2021-06-29  45m added icon to .exe and icon to application.
- 2021-06-28  60m added exit dialog, added background coloring of hud
- 2021-06-27  15m added CI for installer, tweaked github actions
- 2021-06-26  20m started working on installer using inno setup
- 2021-06-25  60m added quick to ini, fixed quick race condition, renamed config default to config release, added tank explosion, updated bullet hit explosion
- 2021-06-24  115m added show fps when true, added fullscreen, width and height support in config
- 2021-06-23  100m fixed several warnings, moved common/non-game specific mods to extensions crate, started working on integration of ini parsing, started working on FPS showing
- 2021-06-22  100m added levels.json loading, added asset_cache, played with hot reloading, played with debugging support, got debug to work
- 2021-06-21  75m added music channel, worked on levels json, worked on json loader since bevy does not support loading raw json out of the box
- 2021-06-20  55m worked on console support
- 2021-06-19  45m worked on console support
- 2021-06-18  85m added githash and build date to splash, converted mp3 to ogg, found better sound effects, added package versioning to splash, started working on music support
- 2021-06-17  45m worked on release build automation
- 2021-06-16  75m worked on adding sounds
- 2021-06-15  75m worked on instructions, added resize constraint, improved bot navigation, fixed some warningstweaked ai attack behavior
- 2021-06-14  100m looked into persistance and state restoring
- 2021-06-13  40m looked into persistance and state restoring
- 2021-06-12  125m reimplemented game director into director + worked on audio playback, improved ai a bit
- 2021-06-11  70m worked on foreground coloring and fade_in_out support, worked on re-implementing game director
- 2021-06-10  85m worked on splash plugin, started refactoring state system using delay plugin abstraction
- 2021-06-09  60m added support for object rotation in map loader, worked a bit on map 3, added new tiles
- 2021-06-08  100m added working map loader with entity spawner, refactored a bit, worked on level system, added working game!
- 2021-06-07  90m worked on console and map loader support
- 2021-06-06  20m refactored tiledmap
- 2021-06-05  5m worked on map support
- 2021-06-04  75m worked on map support, refactored assets folder structure a bit
- 2021-06-03  70m correct tank coloring based upon faction, added quick to game director, started working on tracks animation
- 2021-06-02  100m moved game into game_director plugin, updated graphics for tank to include red tank, added faction component and updated ai to match, removed red tank again adn replaced with gray tank that can be painted
- 2021-06-01  85m worked on app states and in game system, forked rapier plugin to add AppState::InGame, refactored a bit, added success state, worked on camera quirk when starting level second time, looked into crashing issue related to texture
- 2021-05-31  105m worked on hud abstraction, worked on game system, worked on state transitioning
- 2021-05-30  60m worked on ui text components 
- 2021-05-29  15m worked on bot
- 2021-05-28  80m worked on bot + fixed lib name conflic + worked on visible and known enemies 
- 2021-05-27  60m worked on bot
- 2021-05-26  25m worked on bot
- 2021-05-25  50m worked on adding text support to game + refactored
- 2021-05-24  65m played around with v-sync off, tried to to get to compile for wasm, changed resolver to 2.0, fixed issue related to vsync off
- 2021-05-23  40m worked on contect pairs to get effect spawning to work
- 2021-05-22  10m worked on contact pairs to enable effect spawning at contact point
- 2021-05-21  30m worked on effects
- 2021-05-20  30m worked on effects
- 2021-05-19  100m worked on tank movement
- 2021-05-18  70m worked on fixing physics sometimes behaving strange, worked on tank movement and tank system
- 2021-05-17  60m worked on physics system, event plugin and tried to fix/circumvent issue with RustC nightly failing to build sometimes, added functional gun, added health system
- 2021-05-16  55m worked on bullet spawning and hitting, started working on collision events
- 2021-05-14  70m worked on turret system and bullet spawning
- 2021-05-12  15m worked on turret system
- 2021-05-11  35m worked on tilemap plugin
- 2021-05-10  35m worked on tilemap plugin
- 2021-05-07  60m worked on collision, started refactored tilemap to use Builder pattern
- 2021-05-06  20m tilemap now has an invalidate which forces updates to collisions
- 2021-05-05  25m worked on static colliders for tilemap
- 2021-05-04  85m worked on introduction of bevy_rapier2d
- 2021-05-03  10m started working on collision detection
- 2021-05-02  25m projectile now follows the turret
- 2021-05-01  35m worked on mouse system and screen to world, moved cursor support into own system and resource, worked on turret follows mouse pos
- 2021-04-29  30m worked on cursor support, screen to world
- 2021-04-28  45m worked on shooting, refactored tilemap as parent, worked on mouse motion
- 2021-04-27  45m worked on shooting
- 2021-04-24  30m worked on ai system, added tank component, added ramdom bot movement
- 2021-04-23  65m got to compile on new p15 laptop, fixed rotation of turret, worked on getting QuerySets for turret system, fixed some warnings
- 2021-04-22  40m worked on tank turret, fixed issue related to down direction, added simple animation of tracks
- 2021-04-21  40m worked on tank movement, fixed issue with scale propagation, moved factory to file, worked on turret support for tank
- 2021-04-20  5m merged tilemap component and system in same mod
- 2021-04-19  90m worked on textures resource, worked on entity factory
- 2021-04-16  60m worked on pixel perfect scaling, fixed resize artifacts, worked on performance measurements and dev optimization for speed
- 2021-04-15  60m worked on improving build speed through dylib, worked on pixel perfect scaling
- 2021-04-14  60m worked on parent / child ecs, worked on input system, refactored, made movable tank
- 2021-04-13  120m worked on intorducing error handling into systems using anyhow, dropped anyhow since i was not able to put it into main, worked on removing unwraps, worked on getting projection to update without resize
- 2021-04-12  60m reversed readme, refactored name of tilemap render system, refactored a bit, cleaned up some stuff, tilemap tiles are now 1x1 in size, added camera system to control camera and align it to center
- 2021-04-10  40m made a square map with walls, cleaned up unused components, worked on introduction of player, worked on movement_system
- 2021-04-09  60m  played with tilemap, renamed grid to tilemap, renamed project to blueprint3, made a simple tank spritesheet, refactored tiemap construction
- 2021-04-08  60m  updated to bevy 0.5, worked on grid component, added grid_renderer system such that grid can update mesh, added working tilemap
- 2021-03-30  30m implemented very simple hardcoded tilemap
- 2021-03-30  15m played around with 3d api
- 2021-03-29  15m blog skrivning
- 2021-03-28  45m played around with sprite sheet
- 2021-03-28  15m written a bit on a blog post.
- 2021-03-27  60m  initial project setup + systems and some components + rendered a single sprite








