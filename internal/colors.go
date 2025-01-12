package internal

import (
	"fmt"
	"github.com/charmbracelet/lipgloss"
	"github.com/lucasb-eyer/go-colorful"
	"strconv"
)

const (
	slashChar     = "/"
	seperatorChar = "|"
)

var (
	KeywordStyle   = lipgloss.NewStyle().Foreground(lipgloss.Color("211"))
	SubtleStyle    = lipgloss.NewStyle().Foreground(lipgloss.Color("241"))
	SlashStyle     = lipgloss.NewStyle().Foreground(lipgloss.Color("236")).Render(slashChar)
	SeperatorStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("236")).Render(seperatorChar)
	mainStyle      = lipgloss.NewStyle().MarginLeft(2)
)

// Convert a colorful.Color to a hexadecimal format.
func colorToHex(c colorful.Color) string {
	return fmt.Sprintf("#%s%s%s", colorFloatToHex(c.R), colorFloatToHex(c.G), colorFloatToHex(c.B))
}

// Helper function for converting colors to hex. Assumes a value between 0 and
// 1.
func colorFloatToHex(f float64) (s string) {
	s = strconv.FormatInt(int64(f*255), 16)
	if len(s) == 1 {
		s = "0" + s
	}
	return
}
