import { BEATMAP_URL_REGEX } from "../common/constants";
import playPreview from "./canvas";
import * as ojsama from "./ojsama";
import { Beatmap, Renderer } from "../backend/pkg";

const FETCH_ATTEMPTS = 3;
const UNSUPPORTED_GAMEMODE = "Unsupported gamemode!"; // TODO: Add to translations

const containerElement = document.getElementById("container");
const headerElement = document.getElementById("header");
const titleElement = document.querySelector(".song-title");
const artistElement = document.querySelector(".artist");
const difficultyNameElement = document.getElementById("difficulty-name");
const playbackTimeElement = document.getElementById("playback-time");
const progressElement = document.querySelector(".progress");
/** @type {HTMLCanvasElement} */
const canvasElement = document.getElementById("canvas");
const errorElement = document.getElementById("error");

// Set after the extension initializes, used for additional error information.
let previewTime = null;
let libosuBeatmap = null;
let cleanBeatmap = null;
// let renderer = new Renderer(canvasElement);
let pageInfo = {
  isOldSite: null,
  beatmapSetId: null,
  beatmapId: null,
};

function displayError(error) {
  errorElement.innerText = error.message;
  containerElement.classList.toggle("error", true);
  containerElement.classList.toggle("preloading", false);
}

function onReady([, cover]) {
  // Display content since we're done loading all the stuff.
  containerElement.classList.toggle("preloading", false);

  // Set header background
  if (cover) {
    headerElement.style.backgroundImage = `url('${cover.src}')`;
  }

  // Set header text
  titleElement.innerText = libosuBeatmap.title();
  artistElement.innerText = libosuBeatmap.artist();
  difficultyNameElement.innerText = libosuBeatmap.difficulty_name();

  const audio = new Audio();
  audio.volume = 0.45;
  audio.src = `https://b.ppy.sh/preview/${pageInfo.beatmapSetId}.mp3`;
  audio
    .play()
    .then(() =>
      playPreview(
        canvasElement,
        playbackTimeElement,
        progressElement,
        cleanBeatmap,
        previewTime
      )
    )
    .catch(displayError);
  progressElement.addEventListener("pointerdown", () => {
    audio.pause();
  });
}

const fetchBeatmapById = (id) =>
  fetch(`https://osu.ppy.sh/osu/${id}`, { credentials: "include" }).then(
    (res) => res.text()
  );

const getPageInfo = (url, tabId) =>
  new Promise((resolve, reject) => {
    const info = {
      isOldSite: null,
      beatmapSetId: null,
      beatmapId: null,
    };

    const match = url.match(BEATMAP_URL_REGEX);
    info.isOldSite = match[2] !== "beatmapsets";

    if (!info.isOldSite) {
      const beatmapId = match[4];

      if (!beatmapId) {
        throw new Error(UNSUPPORTED_GAMEMODE);
      }

      info.beatmapSetId = match[3];
      info.beatmapId = beatmapId;

      resolve(info);
    } else {
      // Fetch data from the content script so we don't need to fetch the page
      // second time.
      chrome.tabs.sendMessage(
        tabId,
        { action: "GET_BEATMAP_INFO" },
        (response) => {
          if (response.status === "ERROR") {
            reject(response.error);
          } else {
            const { beatmapId, beatmapSetId } = response;
            info.beatmapSetId = beatmapSetId;
            info.beatmapId = beatmapId;

            resolve(info);
          }
        }
      );
    }
  });

const attemptToFetchBeatmap = (id, attempts) =>
  fetchBeatmapById(id).catch((error) => {
    // Retry fetching until no attempts are left.
    if (attempts) return attemptToFetchBeatmap(id, attempts - 1);

    throw error;
  });

const processBeatmap = (rawBeatmap) => {
  libosuBeatmap = Beatmap.new(rawBeatmap);
  previewTime = libosuBeatmap.preview_time();

  const { map } = new ojsama.parser().feed(rawBeatmap);
  cleanBeatmap = map;

  chrome.extension.getBackgroundPage().console.log(cleanBeatmap);

  const supportedGamemodes = [0, 3];
  if (!supportedGamemodes.includes(libosuBeatmap.game_mode())) {
    throw Error(UNSUPPORTED_GAMEMODE);
  }
};

const fetchBeatmapBackground = (beatmapSetId) =>
  new Promise((resolve) => {
    // Preload beatmap cover
    const cover = new Image();
    cover.src = `https://assets.ppy.sh/beatmaps/${beatmapSetId}/covers/cover@2x.jpg`;
    cover.onload = () => resolve(cover);
    cover.onerror = () => resolve();
    cover.onabort = () => resolve();
  });

if (__FIREFOX__) {
  containerElement.classList.toggle("firefox", true);
  document.documentElement.classList.toggle("firefox", true);
}

// Init the extension.
chrome.tabs.query(
  {
    active: true, // Select active tabs
    lastFocusedWindow: true, // In the current window
  },
  ([tab]) => {
    const { url, id } = tab;
    getPageInfo(url, id)
      .then((info) => {
        pageInfo = info;

        return Promise.all([
          attemptToFetchBeatmap(pageInfo.beatmapId, FETCH_ATTEMPTS).then(
            processBeatmap
          ),
          fetchBeatmapBackground(pageInfo.beatmapSetId),
        ]);
      })
      .then(onReady)
      .catch(displayError);
  }
);
