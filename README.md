On-Air - Sonoff
===============

[![Oh my god, it builds. That's amazing. I'm not even mad.](https://travis-ci.org/armyofevilrobots/onair-sonoff.svg?branch=master)](https://travis-ci.org/armyofevilrobots/onair-sonoff)

Y'know when you walk away from the meeting you're currently in (not that I'd EVER be that irresponsible... ahem). 
And then you're running the coffee machine, grinding the beans, clanking around gently swearing to yourself
about running low on sugar, or finding your favourite mug, when...

> Hey, could whoever that is mute?!

They're feigning polite, but you're not fooled. They know exactly who was making the noise. 
There's a little "This jerk is making noise" indicator
right there in the meeting software, beside _YOUR_ name.

If only you had some wireless magic sign to let you know when you forgot to mute!

## TADA!!!

This is just a little toy project that'll connect to ~plantronics hub~
your sound device and start recording, then monitor for whether it's truly
muted (is it making any sounds?) in order to pull the mute status. It currently writes to a [tasmota REST API](https://github.com/arendst/Sonoff-Tasmota) 
on an [Sonoff S26 IoT plug](https://www.itead.cc/sonoff-s26-wifi-smart-plug.html)
to mute or unmute an "on the air" sign like this one: 
[On Air Sign: Aliexpress](https://www.aliexpress.com/item/LB480-On-Air-Recording-Studio-NEW-NR-LED-Neon-Light-Sign-home-decor-crafts/1000006552370.html).

The end result is...

![Animated](https://i.imgur.com/msRk3HK.gif)

Note; this used to talk to the Plantronics REST API, but that's only
available for plantronics, and/or for the Plantronics device. This
method is crusty too, but at least it works on Linux, Win32 and OSX
consistently.

```
onair-sonoff 0.0.1
Derek Anderson <derek@armyofevilrobots.com>
Monitors state of a plantronics headset and sends events to various endpoints.

USAGE:
    onair-sonoff [OPTIONS] --tasmota <tasmota>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>        Sets a custom config file
    -T, --tasmota <tasmota>    The destination url for the tasmota rest api (http://sonoff-on-air.local/)
```

