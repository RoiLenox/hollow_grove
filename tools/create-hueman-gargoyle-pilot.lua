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
local layer = sprite.layers[1]
layer.name = "hueman_gargoyle_pilot"

local images = {sprite.cels[1].image}
for frame_index = 2, 4 do
  local frame = sprite:newEmptyFrame()
  local image = Image(16, 16, ColorMode.RGB)
  image:clear(app.pixelColor.rgba(0, 0, 0, 0))
  local cel = sprite:newCel(layer, frame, image)
  images[frame_index] = cel.image
end
images[1]:clear(app.pixelColor.rgba(0, 0, 0, 0))

local function pixel(image, x, y, color_id)
  local value = colors[color_id]
  if value == nil then
    error("missing constitutional palette color " .. color_id)
  end
  image:putPixel(x, y, value)
end

local function rect(image, x, y, width, height, color_id)
  for py = y, y + height - 1 do
    for px = x, x + width - 1 do
      pixel(image, px, py, color_id)
    end
  end
end

local function pixels(image, points, color_id)
  for _, point in ipairs(points) do
    pixel(image, point[1], point[2], color_id)
  end
end

-- Frame 1: ordinary Hueman. The dark hair, teal mantle, satchel, and Aura
-- jewel are the continuity anchors carried through every later form.
local hueman = images[1]
rect(hueman, 4, 1, 8, 5, "universal.oxford_blue")
rect(hueman, 5, 4, 6, 3, "flynt.powder_blue")
pixels(hueman, {{6, 5}, {9, 5}}, "universal.oxford_blue")
rect(hueman, 3, 7, 10, 6, "universal.oxford_blue")
rect(hueman, 4, 8, 8, 5, "glaushouse.viridian")
rect(hueman, 5, 8, 2, 4, "glaushouse.eton_blue")
rect(hueman, 7, 8, 2, 2, "flynt.tropical_indigo")
rect(hueman, 11, 8, 2, 4, "stonebend.lapis_lazuli")
rect(hueman, 4, 13, 3, 2, "flynt.gunmetal")
rect(hueman, 9, 13, 3, 2, "flynt.gunmetal")
rect(hueman, 3, 15, 10, 1, "universal.midnight_blue")

-- Frame 2: stable grounded Gargoyle trial. A low center of mass and folded
-- shoulder ridges make this visibly a trial form rather than flight.
local gargoyle = images[2]
pixels(gargoyle, {{3, 3}, {12, 3}, {4, 4}, {11, 4}}, "universal.oxford_blue")
rect(gargoyle, 4, 4, 8, 5, "universal.oxford_blue")
rect(gargoyle, 5, 5, 6, 3, "flynt.gunmetal")
pixels(gargoyle, {{6, 6}, {9, 6}}, "flynt.powder_blue")
rect(gargoyle, 2, 8, 12, 5, "universal.oxford_blue")
rect(gargoyle, 3, 9, 10, 4, "stonebend.lapis_lazuli")
rect(gargoyle, 5, 9, 6, 2, "stonebend.air_superiority_blue")
rect(gargoyle, 7, 10, 2, 2, "flynt.tropical_indigo")
rect(gargoyle, 1, 11, 4, 3, "flynt.gunmetal")
rect(gargoyle, 11, 11, 4, 3, "flynt.gunmetal")
rect(gargoyle, 3, 14, 4, 2, "universal.midnight_blue")
rect(gargoyle, 9, 14, 4, 2, "universal.midnight_blue")

-- Frame 3: cleared winged Gargoyle. Wings occupy the full silhouette while
-- the teal mantle and indigo Aura jewel retain Hueman's recognizable center.
local winged = images[3]
pixels(winged, {{0, 3}, {15, 3}, {1, 4}, {14, 4}}, "flynt.powder_blue")
rect(winged, 0, 5, 5, 2, "universal.oxford_blue")
rect(winged, 11, 5, 5, 2, "universal.oxford_blue")
rect(winged, 1, 7, 5, 2, "flynt.gunmetal")
rect(winged, 10, 7, 5, 2, "flynt.gunmetal")
rect(winged, 2, 9, 4, 2, "stonebend.lapis_lazuli")
rect(winged, 10, 9, 4, 2, "stonebend.lapis_lazuli")
rect(winged, 5, 3, 6, 5, "universal.oxford_blue")
rect(winged, 6, 4, 4, 3, "flynt.gunmetal")
pixels(winged, {{6, 5}, {9, 5}}, "flynt.powder_blue")
rect(winged, 5, 8, 6, 5, "universal.oxford_blue")
rect(winged, 6, 8, 4, 4, "glaushouse.viridian")
rect(winged, 7, 9, 2, 2, "flynt.tropical_indigo")
rect(winged, 5, 13, 2, 2, "flynt.gunmetal")
rect(winged, 9, 13, 2, 2, "flynt.gunmetal")
pixels(winged, {{4, 11}, {11, 11}, {3, 12}, {12, 12}}, "flynt.tropical_indigo")

-- Frame 4: safe return. Hueman is ordinary again, visibly tired but intact;
-- the unlocked capability persists in state rather than in the body frame.
local returned = images[4]
rect(returned, 4, 2, 8, 4, "universal.oxford_blue")
rect(returned, 5, 5, 6, 3, "flynt.powder_blue")
pixels(returned, {{6, 6}, {9, 6}}, "universal.midnight_blue")
rect(returned, 3, 8, 10, 5, "universal.oxford_blue")
rect(returned, 4, 9, 8, 4, "glaushouse.brunswick_green")
rect(returned, 5, 9, 2, 3, "glaushouse.viridian")
rect(returned, 7, 9, 2, 2, "flynt.tropical_indigo")
rect(returned, 11, 9, 2, 3, "stonebend.prussian_blue")
rect(returned, 3, 13, 4, 2, "flynt.gunmetal")
rect(returned, 9, 13, 4, 2, "flynt.gunmetal")
rect(returned, 2, 15, 12, 1, "universal.midnight_blue")

for frame_index, tag_name in ipairs({"hueman", "gargoyle", "winged", "return"}) do
  local tag = sprite:newTag(frame_index, frame_index)
  tag.name = tag_name
end

sprite:saveAs(output)
sprite:close()
