# bilui (lib ui)

This project is demo/wip attempt at making a desktop ui framework using rust and sdl2

Plans:
- Boxes
- Layouts
- Text
- Stylesheets
- Yaml support ?

## Questions

#### Layouts

~~As you might have seen layouts are basically a box that matches the size of it's parent and arranges it's children in a specific fashion.  This might be subject to changes in the future as it is not really readable.~~

Problem fixed: Flex layouts are now resizable like ScaledRect!

---------------------------------------------------------------
```
Also that the width is actually determined by the layout's width and not a fractional width like in the flexbox layout in html and css when doing width: 100% on a child.

This could be a different layout or a layout mode for example:

child_space: parent; /* default */
child_space: fractional /* each child has an equal fraction of the parent's width */
```

---------------------------------------------------------------

Problem fixed! Added an option to specify the way that the children's width/height is calculated on a flex layout;

```rust
struct ChildSize {
    Fractional,
    Absolute,
}
```


### The way that margin is handled.

Margin right now is handled in a very strict way that doesn't allow for much creativity.

I'm aware of the fact that I still need to implement the gap property in the layout and it'll probably be a better alternative to style space between elements

I'll also probably add a way to have 4 way margin instead of a 2 way one (top, left, bottom, right instead of width/height).

### Yaml?

I would like in the future to implement a more idiomatic way to create layouts...

I thought that something like yaml (with a similar structure) could be a good way to create layouts

```yaml
main:
    row:
        scaled_rect(50,100):
            flex(column):
                scaled_rect(100, 50)
```

and maybe something like css or idk for styling