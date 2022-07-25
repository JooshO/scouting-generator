<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { desktopDir } from "@tauri-apps/api/path";

  import Adder from "./Adder.svelte";
  import Question from "./Question.svelte";

  import { dndzone } from "svelte-dnd-action";

  // When using the Tauri API npm package:
  import { invoke } from "@tauri-apps/api/tauri";

  let id: number = 2;
  let allQuestions = new Map([
    [
      "prematch",
      [
        {
          label: "Team Number",
          type: 0,
          section: "prematch",
          options: ["PRIMARY KEY"],
          deletable: false,
          id: 0,
        },
        {
          label: "Match Number",
          type: 0,
          section: "prematch",
          options: ["PRIMARY KEY"],
          deletable: false,
          id: 1,
        },
      ],
    ],
    ["auton", []],
    ["teleop", []],
    ["endgame", []],
  ]);
  export const name: string = "Scouting Question Generator";

  /**
   * Delete helper
   * @param label The label that we are removing
   * @param section The section we expect it to be in
   */
  function del(label: string, section: string) {
    // grab original size
    let originalSize = allQuestions.get(section).length;

    // filter to delete, and set
    allQuestions.set(
      section,
      allQuestions[section].filter(
        (a) => a.label !== label || a.section !== section
      )
    );
    allQuestions[section] = allQuestions.get(section);

    // if this element was moved, del breaks and looks at the original section
    // I'm not sure how to fix this, so if we don't delete anything, we'll just try the other sections.
    if (originalSize == allQuestions.get(section).length) {
      allQuestions.forEach((value, key) => {
        allQuestions.set(
          key,
          allQuestions[key].filter((a) => a.label !== label)
        );
        allQuestions[key] = allQuestions.get(key);
      });
    }
  }

  function questionMoveHelper(target: string) {
    for (let index = 0; index < allQuestions.get(target).length; index++) {
      var old: string = allQuestions.get(target)[index].section;
      if (old != target) {
        allQuestions.get(target)[index].section = target;
        allQuestions.get(target)[index].id =
          -allQuestions.get(target)[index].id;
      }
    }
  }

  //#region helper functions for drag and drop
  function finalizePrematch(e) {
    let target = "prematch";
    questionMoveHelper(target);
    allQuestions.set(target, e.detail.items);
    allQuestions[target] = allQuestions.get(target);
  }
  function considerPrematch(e: { detail: { items: any } }) {
    let target = "prematch";
    questionMoveHelper(target);
    allQuestions.set(target, e.detail.items);
    allQuestions[target] = allQuestions.get(target);
  }

  function finalizeAuton(e) {
    let target = "auton";
    questionMoveHelper(target);
    allQuestions.set(target, e.detail.items);
    allQuestions[target] = allQuestions.get(target);
  }
  function considerAuton(e) {
    let target = "auton";
    questionMoveHelper(target);
    allQuestions.set(target, e.detail.items);
    allQuestions[target] = allQuestions.get(target);
  }

  function finalizeTeleop(e) {
    let target = "teleop";
    questionMoveHelper(target);
    allQuestions.set(target, e.detail.items);
    allQuestions[target] = allQuestions.get(target);
  }
  function considerTeleop(e) {
    let target = "teleop";
    questionMoveHelper(target);
    allQuestions.set(target, e.detail.items);
    allQuestions[target] = allQuestions.get(target);
  }

  function finalizeEndgame(e) {
    let target = "endgame";
    questionMoveHelper(target);
    allQuestions.set(target, e.detail.items);
    allQuestions[target] = allQuestions.get(target);
  }
  function considerEndgame(e) {
    let target = "endgame";
    questionMoveHelper(target);
    allQuestions.set(target, e.detail.items);
    allQuestions[target] = allQuestions.get(target);
  }
  //#endregion

  // add callback to insert the created object into the appropriate list
  function add(
    label: string,
    type: number,
    section: string,
    options: string[]
  ) {
    allQuestions.set(section, [
      ...allQuestions.get(section),
      {
        label: label,
        type: type,
        section: section,
        options: options,
        deletable: true,
        id: id++,
      },
    ]);
    allQuestions[section] = allQuestions.get(section);
  }

  /**
   * invokes rust command to write our data to an output file
   * Will let you pick location, but not file name, bc that's hardcoded in the scouting app
   */
  async function saveFile() {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: await desktopDir(),
    });
    if (selected === null) {
      return;
    } else {
      invoke("write_output", {
        path: selected,
        data: JSON.stringify(allQuestions),
      });
      // user selected a single directory
    }
  }
  // writeFile({ path: value, contents: JSON.stringify(audios) });
</script>

<main class="box">
  <div>
    <h2>Prematch</h2>
    <!-- // https://github.com/isaacHagoel/svelte-dnd-action -->
    <div
      class="slot"
      use:dndzone={{ items: allQuestions.get("prematch") }}
      on:consider={considerPrematch}
      on:finalize={finalizePrematch}
    >
      {#each allQuestions.get("prematch") as question (question.id)}
        <Question
          label={question.label}
          options={question.options}
          type={question.type}
          section={question.section}
          onDelete={del}
          deletable={question.deletable}
        />
      {/each}
    </div>
    <h2>Auton</h2>
    <div
      class="slot"
      use:dndzone={{ items: allQuestions.get("auton") }}
      on:consider={considerAuton}
      on:finalize={finalizeAuton}
    >
      {#each allQuestions.get("auton") as question (question.id)}
        <Question
          label={question.label}
          options={question.options}
          type={question.type}
          section={question.section}
          onDelete={del}
          deletable={question.deletable}
        />
      {/each}
    </div>
    <h2>Teleop</h2>
    <div
      class="slot"
      use:dndzone={{ items: allQuestions.get("teleop") }}
      on:consider={considerTeleop}
      on:finalize={finalizeTeleop}
    >
      {#each allQuestions.get("teleop") as question (question.id)}
        <Question
          label={question.label}
          options={question.options}
          type={question.type}
          section={question.section}
          onDelete={del}
          deletable={question.deletable}
        />
      {/each}
    </div>
    <h2>Endgame</h2>
    <div
      class="slot"
      use:dndzone={{ items: allQuestions.get("endgame") }}
      on:consider={considerEndgame}
      on:finalize={finalizeEndgame}
    >
      {#each allQuestions.get("endgame") as question (question.id)}
        <Question
          label={question.label}
          options={question.options}
          type={question.type}
          section={question.section}
          onDelete={del}
          deletable={question.deletable}
        />
      {/each}
    </div>
  </div>
  <Adder {add} save={saveFile} />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h2 {
    text-align: left;
    margin: 0.5em;
  }

  .slot {
    min-height: 3.1em;
  }

  .box {
    display: grid;
    grid-auto-flow: row;
    grid-template-columns: 75% 25%;
    height: 95%;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
