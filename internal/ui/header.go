package ui

import (
	"fmt"

	"github.com/lazytf/internal"
)

func GetHeader() string {
	return fmt.Sprintf("%s%s%s", internal.KeywordStyle.Render("Project Name"), internal.SlashStyle, internal.SubtleStyle.Render("Workspace Name"))
}

func GetFooter() string {
	actions := []struct {
		key   string
		value string
	}{
		{"Apply", "A"},
		{"Plan", "P"},
		{"Destroy", "D"},
		{"State", "s"},
		{"Bindings", "?"},
		{"Cancel", "Esc"},
		{"Quit", "q"},
	}

	var headerString string
	for _, action := range actions {
		headerString += fmt.Sprintf(" %s %s %s", internal.KeywordStyle.Render(action.key), internal.SubtleStyle.Render(action.value), internal.SeperatorStyle)
	}
	return headerString
}
