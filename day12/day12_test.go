package main

import "testing"

func TestSample(t *testing.T) {
	const s = `F10
		N3
		F7
		R90
		F11`

	actions := parse(s)

	// Another reason go sucks: no assert... this is so wordy
	if actions[1].a != 'N' ||  actions[1].v != 3 {
		t.Errorf("Expected N 3, got %c %d", actions[1].a, actions[1].v)
	}

	m := part1move(actions)
	if m != 25 {
		t.Errorf("Expected manhatten dist 25, got %d", m)
	}

	dx, dy := rotate(1, 0, 90)
	if dx != 0 || dy != -1 {
		t.Errorf("Expected 0, -1 - got %d, %d", dx, dy)
	}

	dx, dy = rotate(3, 1, 180)
	if dx != -3 || dy != -1 {
		t.Errorf("Expected -3, -1, got %d, %d", dx, dy)
	}

	dx, dy = rotate(5, 2, -90)
	if dx != -2 || dy != 5 {
		t.Errorf("Expected -2, 5, got %d, %d", dx, dy)
	}

	m2 := part2move(actions)
	if m2 != 286 {
		t.Errorf("Expected manhatten dist 286, got %d", m)
	}
}
