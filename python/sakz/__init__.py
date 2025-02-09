from ._sakz import ffi, lib


class Figure:
    def __init__(self, title: str) -> None:
        _title = ffi.new("char[]", title.encode())
        self.fig = lib.sakz_figure_new(_title)

    def title(self, title: str) -> "Figure":
        _title = ffi.new("char[]", title.encode())
        lib.sakz_figure_title(self.fig, _title)
        return self

    def save(self, filename: str) -> None:
        _filename = ffi.new("char[]", filename.encode())
        result = lib.sakz_figure_save(self.fig, _filename)
        if result != 0:
            raise RuntimeError("Failed to save figure")
