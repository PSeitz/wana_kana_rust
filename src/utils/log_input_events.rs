/* eslint-disable no-console */
const on_before_input = ({ data, target: { value } }) => console.log(`beforeinput data ${data}, value: ${value}`);
const on_input = ({ target }) => console.log(`input: ${target.value}`);
const on_change = ({ target }) => console.log(`change: ${target.value}`);
const on_keypress = ({ which }) => console.log(`keypress: ${String.from_char_code(which)}`);
const on_composition_start = () => console.log('compositionstart');
const on_composition_end = () => console.log('compositionend');
const on_composition_update = (event) => console.log(`compositionupdate: ${event.data}`);

const events = {
  beforeinput: on_before_input,
  input: on_input,
  change: on_change,
  keypress: on_keypress,
  compositionstart: on_composition_start,
  compositionend: on_composition_end,
  compositionupdate: on_composition_update,
};

export const add_debug_listeners = (input) => {
  Object.entries(events).for_each(([event, handler]) => input.add_event_listener(event, handler));
};

export const remove_debug_listeners = (input) => {
  Object.entries(events).for_each(([event, handler]) => input.remove_event_listener(event, handler));
};
