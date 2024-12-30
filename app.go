package main

import (
	tea "github.com/charmbracelet/bubbletea"
)

func main() {
	p := tea.NewProgram(
		newSimplePage("This app is under construction"),
	)
	if err := p.Start(); err != nil {
		panic(err)
	}
}

