package ui

import (
	"github.com/charmbracelet/lipgloss"
	"github.com/lazytf/models"
)

func Layout(m models.Model, height int) string {
	columnOneWidthUnits := 2
	columnTwoWidthUnits := 4

	unitWidthSize := m.Width / (columnOneWidthUnits + columnTwoWidthUnits)

	columnOne := lipgloss.NewStyle().
		Align(lipgloss.Left).
		Width(unitWidthSize*columnOneWidthUnits).
		Height(height).
		PaddingTop(-10).
		Border(lipgloss.RoundedBorder(), true, true, true, true).
		Render("Left")

	columnTwo := lipgloss.NewStyle().
		Align(lipgloss.Left).
		Width(unitWidthSize*columnTwoWidthUnits).
		Height(height).
		Border(lipgloss.RoundedBorder(), true, true, true, true).
		Render("Right")

	return lipgloss.JoinHorizontal(lipgloss.Top, columnOne, columnTwo)
}
