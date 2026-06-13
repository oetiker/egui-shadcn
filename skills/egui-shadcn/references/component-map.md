# shadcn component → egui widget

| shadcn | module widget |
|---|---|
| `<Button variant size>` | `components::button::Button::new(t).variant(..).size(..)` via `ui.add(..)` — variants Default/Destructive/Outline/Secondary/Ghost/Link; sizes Sm/Default/Lg/Icon |
| `<Input>` | `components::input::Input::new(&mut s).hint(..).password(..).max_width(..)` via `ui.add(..)` |
| `<Label>` | `components::label::label(ui, "..")` |
| description text | `components::label::description(ui, "..")` |
| `<Card>` + Header/Title/Description | `layout::card(ui, |ui| ...)` + `components::card::{card_title, card_description}` |
| `<Tabs>` | `components::tabs::tab_bar(ui, &mut active, &[..])` |
| `<Switch>` | `components::switch::toggle(ui, &mut on)` |
| `<Checkbox>` | `components::checkbox::checkbox(ui, &mut checked)` |
| `<Select>` | `components::select::select(ui, "id", &mut idx, &[..])` |
| `<Separator>` | `components::separator::separator(ui)` |
| `<Badge variant>` | `components::badge::badge(ui, "..", BadgeVariant::..)` |

Not yet ported (add when needed): Dialog, Popover, Table, Tooltip, DropdownMenu,
Accordion, gradients. Build them as new files under `components/` following the
custom-paint pattern in `button.rs`.
