# Beyhl Synth-Station

![Beyhl Synth-Station](Beyhl-Synth-Station.png)

Ein Synthesizer für Linux und Windows mit eigenem Klangerzeuger („Beyhl-Kern“) und großer Bedienoberfläche,
entwickelt von Klaus Beyhl (Beyhl Software) zusammen mit Claude Opus 5.5. Kostenlos.

## ⬇ Herunterladen

**[⬇ Für Linux](https://github.com/SualkKlaus/Beyhl-Synth-Station/releases/download/v30/Beyhl-Synth-Station-V30)**

**[⬇ Für Windows](https://github.com/SualkKlaus/Beyhl-Synth-Station/releases/download/v31-windows-test/Beyhl-Synth-Station-V31.exe)**

**[⬇ Klangbänke (amsynth, GPL 2)](https://github.com/SualkKlaus/Beyhl-Synth-Station/releases/download/v30/Klangbaenke-amsynth.zip)** – entpacken und die .bank-Dateien neben das Programm legen.

## Video

[Video auf YouTube](https://www.youtube.com/watch?v=G9VVlh5ALDQ)

## Was er kann

- Eigener Klangkern: 2 Oszillatoren plus LFO, Filter mit Moog-Kaskade, Hüllkurven mit Kondensator-Verlauf
- Studio-Effekte direkt im Kern: Chorus, Studio-Hall mit 7 Räumen, Echo, 4-Band-EQ
- Arpeggiator mit fertigen Werks-Mustern, 30 Szenen, Oszilloskop, Grundstimmung 415–466 Hz, KI-Steuerung
- Spielbar über jedes MIDI-Keyboard oder die Bildschirm-Klaviatur

## Starten

**Linux:** Datei in einen eigenen Ordner legen, Rechtsklick → Eigenschaften → „Als Programm ausführen“ ankreuzen, per Doppelklick starten. Voraussetzung: 64-Bit-Linux, aktuelles System (z. B. Ubuntu 24.04 oder neuer).

**Windows:** Datei in einen eigenen Ordner legen und per Doppelklick starten. Warnt Windows vor einer unbekannten App: „Weitere Informationen“ → „Trotzdem ausführen“. Voraussetzung: 64-Bit-Windows.

## Klangbänke

Der Synth spielt amSynth-Klangbänke (Dateien mit Endung .bank).
- Ist amsynth installiert, werden dessen Bänke automatisch benutzt.
- Sonst die Bänke einfach in denselben Ordner wie das Programm legen – beliebig viele, auch eigene.
- Die Werksbänke gibt es oben unter „Klangbänke“ als Zip; Herkunft: https://github.com/amsynth/amsynth, Lizenz GNU GPL 2.

Eigene Arp-Muster und Szenen speichert das Programm in seinem Ordner.

## Ton

Der Ton geht automatisch an den Standard-Ausgang von Linux (Toneinstellungen).
Unter PipeWire erscheint der Synth als eigene Tonquelle „beyhl_kern_v5“ und kann von dort an jedes Ziel geleitet werden.
Unter Windows geht der Ton an den Standard-Ausgang aus den Windows-Toneinstellungen.

## Lizenz

Kostenlos benutzbar und weitergebbar, Einzelheiten in LICENSE.
Hall und Chorus (calf.rs) stehen unter der GNU LGPL 2.1, der Quelltext liegt in diesem Repository.
