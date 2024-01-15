# Game of Life in Rust
Umsetzung des Game of Life in Rust.

## Konfigurationsdatei
Die config.toml Datei steuert die die Parameter für die Simulation des "Game of Life". 
### Die Simulation kann in verschiedenen Modi ausgeführt werden, die die Benutzeroberfläche und das Verhalten beeinflussen:

```toml
interface = "Gui"  # Mögliche Werte: Gui, Console, Speed, SuperSpeed
max_fps = 60 # Anzahl der Bilder pro Sekunde
ups = 10 # Anzahl der Updates des Gamegrids pro Sekunde
```

- Gui - Grafische Darstellung des GoL
- Console - Konsolen Darstellung des GoL
- Speed - Durchlauf von 10000 Generationen mit Konsolenausgabe
- Superspeed - Durchlauf von 10000 Generationen ohne Konsolenausgabe

### Anfangszustand der Zellen bestimmen:

```toml
initial_cells = [
  # Blinker
  [1, 1],
  [1, 2],
  [1, 3]
]
```

### Die Größe des Gitters, auf dem die Zellen platziert sind, wird durch die folgenden Parameter definiert:

```toml
[grid_size]
width = 30
height = 30
```

## GUI
Tastenfunktionen im Spiel:

Spiel beenden:
- Drücke die "Q"-Taste, um das Spiel zu beenden.

Spiel pausieren:
- Drücke die Leertaste (Spacebar), um das Spiel zu pausieren.
