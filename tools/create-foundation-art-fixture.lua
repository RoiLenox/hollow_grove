local output = app.params["output"]
local palette_path = app.params["palette"]
if output == nil or output == "" or palette_path == nil or palette_path == "" then
  error("output and palette script parameters are required")
end

local palette_file = assert(io.open(palette_path, "r"))
local palette_document = json.decode(palette_file:read("*a"))
palette_file:close()

local colors = {}
for _, record in ipairs(palette_document.colors) do
  local hex = string.gsub(record.hex, "#", "")
  colors[record.id] = app.pixelColor.rgba(
    tonumber(string.sub(hex, 1, 2), 16),
    tonumber(string.sub(hex, 3, 4), 16),
    tonumber(string.sub(hex, 5, 6), 16),
    255
  )
end

local sprite = Sprite(16, 16, ColorMode.RGB)
sprite.layers[1].name = "foundation_pipeline_probe"
local image = sprite.cels[1].image
image:clear(app.pixelColor.rgba(0, 0, 0, 0))

local function pixel(x, y, color_id)
  local value = colors[color_id]
  if value == nil then
    error("missing constitutional palette color " .. color_id)
  end
  image:putPixel(x, y, value)
end

local function pixels(points, color_id)
  for _, point in ipairs(points) do
    pixel(point[1], point[2], color_id)
  end
end

-- A tiny open return arc with a living sprout, adapted from the generated
-- reference while remaining exact, transparent, and constitutionally colored.
pixels({{7, 2}, {8, 2}, {7, 3}, {8, 3}, {7, 4}, {8, 4}}, "flynt.powder_blue")
pixels({{5, 3}, {6, 3}, {9, 3}, {10, 3}, {6, 4}, {9, 4}}, "glaushouse.eton_blue")
pixels({{3, 5}, {4, 5}, {11, 5}, {12, 5}, {2, 6}, {13, 6}, {1, 7}, {14, 7},
        {1, 8}, {14, 8}, {0, 9}, {15, 9}, {0, 10}, {15, 10}, {1, 11}, {14, 11},
        {2, 12}, {13, 12}, {3, 13}, {4, 13}, {11, 13}, {12, 13}},
       "universal.oxford_blue")
pixels({{4, 6}, {11, 6}, {3, 7}, {12, 7}, {2, 8}, {13, 8}, {2, 9}, {13, 9},
        {2, 10}, {13, 10}, {3, 11}, {12, 11}, {4, 12}, {11, 12}},
       "flynt.powder_blue")
pixels({{5, 12}, {6, 12}, {9, 12}, {10, 12}, {5, 13}, {6, 13}, {9, 13}, {10, 13}},
       "flynt.tropical_indigo")

local tag = sprite:newTag(1, 1)
tag.name = "idle"
sprite:saveAs(output)
sprite:close()
