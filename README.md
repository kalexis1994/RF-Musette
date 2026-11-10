# RF-Musette

![RF-Musette instrument](package/branding/banner.png)

RF-Musette is a physically modelled accordion for RackForge. Air from the bellows makes the reeds vibrate. The register selects which reeds sound together, and slight tuning differences between reeds create the pulsing sound of a musette.

## Install

RF-Musette is included in the RackForge Standard bundle. To install it separately, download `RF-Musette.rfplugin` and choose **Install plugin** in RackForge's **Plugin Manager**. [Download RF-Musette](https://github.com/kalexis1994/RF-Musette/releases/latest)

## Choose a register and play

Open **RF-Musette** in PLAY and choose from 20 factory programs. Try **Musette Paris** for a wide, wet sound, **Irish Swing** for a slower beat between reeds, and **Jazz** for a darker, drier sound.

- **Register** changes the blend of reeds.
- **Tremolo** changes the tuning spread between reeds that sound together. Listen to the change on a held note with a multi-reed register selected.
- **Cassotto** brings in the darker sound of the tone chamber.
- **Microphones** changes the perspective of the sound.

## Control the bellows and left hand

With **Key Touch** on, key velocity controls how far each key opens and the bellows rest at normal pressure. Turn it off to open the keys fully and shape the bellows with the mod wheel (CC1) or an expression pedal (CC11). **Bellows Direction** can also be controlled with CC80. Values below 64 pull and values from 64 push.

With **Left Hand** on, play treble above the keyboard split and use the lower keys for bass and chords. The default **Split Point** is F3. Use **Bass System** to choose between Stradella and free bass. An accordion controller can address treble on MIDI channel 1, bass on channel 2 and chords on channel 3. [Controller mapping guide](docs/RACKFORGE_CONTROL_MAPPING.md)

The reed and bellows model draws on published accordion acoustics and free-reed measurements. [Sources and credits](docs/SOURCES.md)

## License

RF-Musette is distributed under GPL-2.0-or-later. [License](LICENSE) · [Notices](NOTICE.md)
