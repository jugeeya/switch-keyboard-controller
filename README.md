# switch-keyboard-controller

Control Switch games with your keyboard using software mods.

You can edit the configuration after running the mod once or manually creating the file at `sd:/switchkeyboard/config.toml`.

The default TOML is as follows:
```toml
LSTICK_UP = ["W"]
LSTICK_LEFT = ["A"]
LSTICK_DOWN = ["S"]
LSTICK_RIGHT = ["D"]
LSTICK = []
RSTICK_UP = ["G"]
RSTICK_LEFT = ["V"]
RSTICK_DOWN = ["B"]
RSTICK_RIGHT = ["N"]
RSTICK = []
DUP = ["UPARROW"]
DLEFT = ["LEFTARROW"]
DDOWN = ["DOWNARROW"]
DRIGHT = ["RIGHTARROW"]
A = ["J"]
B = ["K"]
X = ["L"]
Y = ["I"]
L = ["Y"]
ZL = ["U"]
R = ["O"]
ZR = ["P"]
PLUS = ["RETURN"]
MINUS = ["QUOTE"]
TILT1 = ["LEFTSHIFT"]
TILT2 = ["RIGHTSHIFT"]
X1 = []
X2 = []
X3 = []
Y1 = []
Y2 = []
Y3 = []
TILT1_MODIFIER = 0.5
TILT2_MODIFIER = 0.75
X1_MODIFIER = 0.33
X2_MODIFIER = 0.5
X3_MODIFIER = 0.75
Y1_MODIFIER = 0.33
Y2_MODIFIER = 0.5
Y3_MODIFIER = 0.75

# Available Options for Keyboard Keys:
#
# https://github.com/jugeeya/switch-keyboard-controller/blob/master/src/keyboard.rs#L9-L140
```


## Build

### Prerequisites:
- [Skyline](https://github.com/shadowninja108/Skyline)
- [NRO Hook Plugin](https://github.com/ultimate-research/nro-hook-plugin)

Please see [cargo-skyline](https://github.com/jam1garner/cargo-skyline) for detailed steps. We'll just need to `cargo skyline build` here.