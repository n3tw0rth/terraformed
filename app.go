package main

import (
	"fmt"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"

	// "github.com/fogleman/ease"

	"github.com/lazytf/internal"
	"github.com/lazytf/internal/ui"
)

func main() {
	initialModel := model{0, false, 10, 0, 0, false, false, 0, 0}
	p := tea.NewProgram(initialModel, tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Println("could not start program:", err)
	}
}

type (
	tickMsg  struct{}
	frameMsg struct{}
)

func tick() tea.Cmd {
	return tea.Tick(time.Second, func(time.Time) tea.Msg {
		return tickMsg{}
	})
}

func frame() tea.Cmd {
	return tea.Tick(time.Second/60, func(time.Time) tea.Msg {
		return frameMsg{}
	})
}

type model struct {
	Choice        int
	Chosen        bool
	Ticks         int
	Frames        int
	Progress      float64
	Loaded        bool
	Quitting      bool
	height, width int
}

func (m model) Init() tea.Cmd {
	return tick()
}

// Main update function.
func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	// Make sure these keys always quit
	if msg, ok := msg.(tea.KeyMsg); ok {
		k := msg.String()
		if k == "q" || k == "esc" || k == "ctrl+c" {
			m.Quitting = true
			return m, tea.Quit
		}
	}

	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.Type {
		case tea.KeyCtrlC:
			return m, tea.Quit
		}
	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
	}
	return m, nil
}

// The main view, which just calls the appropriate sub-view
func (m model) View() string {
	header := lipgloss.NewStyle().
		Align(lipgloss.Left).
		Width(m.width).
		Border(lipgloss.NormalBorder(), false, false, true, false).
		Render(ui.GetHeader())

	footer := lipgloss.NewStyle().
		Align(lipgloss.Left).
		Width(m.width).
		Render(ui.GetFooter())

	content := lipgloss.NewStyle().
		Width(m.width).
		Height(m.height-lipgloss.Height(header)-lipgloss.Height(footer)).
		Align(lipgloss.Center, lipgloss.Center).
		Render(internal.KeywordStyle.Render("Some Text Goes Here"))

	return lipgloss.JoinVertical(lipgloss.Top, header, content, footer)
}
