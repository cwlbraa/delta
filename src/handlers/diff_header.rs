        let mut handled_line = false;
        // In color_only mode, raw_line's structure shouldn't be changed.
        // So it needs to avoid fn _handle_diff_header_header_line
        // (it connects the plus_file and minus_file),
        // and to call fn handle_generic_diff_header_header_line directly.
        if self.config.color_only {
            write_generic_diff_header_header_line(
                &self.line,
                &self.raw_line,
                &mut self.painter,
                &mut self.mode_info,
                self.config,
            )?;
            handled_line = true;
        }
        Ok(handled_line)
        // In color_only mode, raw_line's structure shouldn't be changed.
        // So it needs to avoid fn _handle_diff_header_header_line
        // (it connects the plus_file and minus_file),
        // and to call fn handle_generic_diff_header_header_line directly.
        if self.config.color_only {
            write_generic_diff_header_header_line(
                &self.line,
                &self.raw_line,
                &mut self.painter,
                &mut self.mode_info,
                self.config,
            )?;
            handled_line = true
            self.handled_diff_header_header_line_file_pair = self.current_file_pair.clone()