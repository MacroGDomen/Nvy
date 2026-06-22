import { describe, expect, it } from "vitest";
import { filterActresses } from "./actressFilter";
import { rotateItems } from "./carousel";
import {
  buildStage10RecommendationPayload,
  buildStage10ScaleDataset,
} from "./stage10ScaleData";
import { filterVideos } from "./videoSearch";

describe("stage 10 scale and regression checks", () => {
  const dataset = buildStage10ScaleDataset();

  it("builds a 1999-record local scale dataset", () => {
    expect(dataset.videos).toHaveLength(1400);
    expect(dataset.actresses).toHaveLength(599);
    expect(dataset.videos.length + dataset.actresses.length).toBe(1999);
  });

  it("keeps video search usable with 1999 records", () => {
    const startedAt = performance.now();
    const result = filterVideos(dataset.videos, {
      searchText: "SCL-0120 + #按摩 + #[22-33]",
      workType: "all",
      sortMode: "updatedDesc",
      tagsByVideoId: dataset.tagsByVideoId,
      actressesByVideoId: dataset.actressesByVideoId,
    });
    const durationMs = performance.now() - startedAt;

    expect(result.length).toBeGreaterThan(0);
    expect(result.every((video) => video.code.includes("SCL-0120"))).toBe(true);
    expect(durationMs).toBeLessThan(1000);
  });

  it("keeps actress text, tag and cup filtering usable with 1999 records", () => {
    const startedAt = performance.now();
    const nameResult = filterActresses(dataset.actresses, {
      searchText: "scale-actress-120",
      selectedCupSizes: [],
      tagsByActressId: dataset.tagsByActressId,
    });
    const tagResult = filterActresses(dataset.actresses, {
      searchText: "#温柔",
      selectedCupSizes: ["F", "G", "H"],
      tagsByActressId: dataset.tagsByActressId,
    });
    const durationMs = performance.now() - startedAt;

    expect(nameResult).toHaveLength(1);
    expect(tagResult.length).toBeGreaterThan(0);
    expect(tagResult.every((actress) => ["F", "G", "H"].includes(actress.cupSize ?? ""))).toBe(true);
    expect(durationMs).toBeLessThan(1000);
  });

  it("rotates large home carousels without changing item count or order rules", () => {
    const startedAt = performance.now();
    let videoWindow = dataset.videos.slice(0, 6);
    let actressWindow = dataset.actresses.slice(0, 7);

    for (let offset = 0; offset < 3000; offset += 1) {
      videoWindow = rotateItems(dataset.videos, offset, 6);
      actressWindow = rotateItems(dataset.actresses, offset, 7);
    }

    const durationMs = performance.now() - startedAt;

    expect(videoWindow).toHaveLength(6);
    expect(actressWindow).toHaveLength(7);
    expect(rotateItems(dataset.videos, -1, 2).map((video) => video.id)).toEqual([
      "scale-video-1400",
      "scale-video-1",
    ]);
    expect(durationMs).toBeLessThan(500);
  });

  it("keeps recommendation payload free of secrets and local absolute paths", () => {
    const payload = buildStage10RecommendationPayload(dataset);
    const serialized = JSON.stringify(payload);

    expect(serialized).not.toMatch(/password|passwordHash|apiKey|api_key|\$argon2|sk-/i);
    expect(serialized).not.toMatch(/[A-Z]:\\\\/);
    expect(serialized).not.toContain("absolutePath");
    expect(payload.videos[0]).not.toHaveProperty("coverPath");
    expect(payload.actresses[0]).not.toHaveProperty("avatarPath");
  });
});
