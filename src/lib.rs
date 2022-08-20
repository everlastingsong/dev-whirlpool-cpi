anchor_gen::generate_cpi_interface!(
  idl_path = "src/artifacts/idl.json",
  target_anchor_version = "0.21.0",
  zero_copy(TickArray, Tick),
  packed(TickArray, Tick)
);

impl Default for state::TickArray {
  fn default() -> Self {
      Self {
          start_tick_index: Default::default(),
          ticks: [Default::default(); 88],
          whirlpool: Default::default(),
      }
  }
}

declare_id!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
