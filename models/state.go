package models

type Model struct {
	Choice        int
	Chosen        bool
	Ticks         int
	Frames        int
	Progress      float64
	Loaded        bool
	Quitting      bool
	Height, Width int
}
