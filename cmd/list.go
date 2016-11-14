package cmd

import (
	"github.com/olekukonko/tablewriter"

	"os"
	"strings"
)

func List() {
	w := loadWorkspace()
	table := tablewriter.NewWriter(os.Stdout)
	table.SetBorder(false)
	table.SetHeader([]string{"project", "tags", "tasks"})

	for _, p := range w.Projects {
		tasks := []string{}
		for t, _ := range p.Tasks {
			if !strings.HasPrefix(t, "_") {
				tasks = append(tasks, t)
			}
		}
		if len(tasks) > 0 {
			table.Append([]string{p.Name, strings.Join(p.Tags, ", "), strings.Join(tasks, ", ")})
		}
	}

	table.Render()
}
