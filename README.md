# Beyhl Synth-Station

![Beyhl Synth-Station](Beyhl-Synth-Station.png)

Ein Synthesizer für Linux mit eigenem Klangerzeuger („Beyhl-Kern“) und großer Bedienoberfläche,
entwickelt von Klaus Beyhl (Beyhl Software) zusammen mit Claude Opus 5.5. Kostenlos.

## Video

[Video auf YouTube](https://www.youtube.com/watch?v=G9VVlh5ALDQ)

## Was er kann

- Eigener Klangkern: 2 Oszillatoren plus LFO, Filter mit Moog-Kaskade, Hüllkurven mit Kondensator-Verlauf
- Studio-Effekte direkt im Kern: Chorus, Studio-Hall mit 7 Räumen, Echo, 4-Band-EQ
- Arpeggiator mit fertigen Werks-Mustern, 30 Szenen, Oszilloskop, Grundstimmung 415–466 Hz, KI-Steuerung
- Spielbar über jedes MIDI-Keyboard oder die Bildschirm-Klaviatur

## Herunterladen und starten

Ein einziges Programm, nichts zu installieren oder zu bauen.

1. Rechts unter „Releases“ die Datei **Beyhl-Synth-Station-V30** herunterladen.
2. In einen eigenen Ordner legen, z. B. „Synth“.
3. Einmal ausführbar machen: Rechtsklick → Eigenschaften → „Als Programm ausführen“
   (oder im Terminal: `chmod +x Beyhl-Synth-Station-V30`).
4. Per Doppelklick starten.

Voraussetzung: 64-Bit-Linux, aktuelles System (z. B. Ubuntu 24.04 oder neuer).

## Klangbänke

Der Synth spielt amSynth-Klangbänke (Dateien mit Endung .bank).
- Ist amsynth installiert, werden dessen Bänke automatisch benutzt.
- Sonst die Bänke einfach in denselben Ordner wie das Programm legen – beliebig viele, auch eigene.
- Die Werksbänke gibt es kostenlos hier: https://github.com/amsynth/amsynth/tree/develop/data/banks

Eigene Arp-Muster und Szenen speichert das Programm in seinem Ordner.

## Ton

Der Ton geht automatisch an den Standard-Ausgang von Linux (Toneinstellungen).
Unter PipeWire erscheint der Synth als eigene Tonquelle „beyhl_kern_v5“ und kann von dort an jedes Ziel geleitet werden.

## Lizenz

Kostenlos benutzbar und weitergebbar, Einzelheiten in LICENSE.
Hall und Chorus (calf.rs) stehen unter der GNU LGPL 2.1, der Quelltext liegt in diesem Repository.
