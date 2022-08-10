<script lang="ts">
  // built in libraries used for file writing
  import { open } from "@tauri-apps/api/dialog";
  import { desktopDir } from "@tauri-apps/api/path";
  import { invoke } from "@tauri-apps/api/tauri";

  // my own elements, the section to add questions and the question blocks respectively
  import Adder from "./Adder.svelte";
  import Question from "./Question.svelte";

  // drag and drop zone, from https://github.com/isaacHagoel/svelte-dnd-action
  import { dndzone } from "svelte-dnd-action";

  // incrementing question id that we use to give each question a unique ID
  let id: number = 3;

  // define our orignal map of questions with Match and Team number baked in
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
    [
      "pit",
      [
        {
          label: "Team Number",
          type: 0,
          section: "pit",
          options: ["PRIMARY KEY"],
          deletable: false,
          id: 2,
        },
      ],
    ],
  ]);

  /**
   * Delete helper
   * @param label The label that we are removing
   * @param section The section we expect it to be in
   */
  function del(id: number, section: string) {
    // grab original size
    let originalSize = allQuestions.get(section).length;

    // filter to delete, and set (setting with = causes svelte to reload the list and display it's updated version)
    allQuestions.set(
      section,
      allQuestions[section].filter((a) => a.id !== id)
    );
    allQuestions[section] = allQuestions.get(section);

    // if this element was moved, del breaks and looks at the original section
    // I'm not sure how to fix this, so if we don't delete anything, we'll just try the other sections.
    if (originalSize == allQuestions.get(section).length) {
      allQuestions.forEach((value, key) => {
        allQuestions.set(
          key,
          allQuestions[key].filter((a) => a.id !== id)
        );
        allQuestions[key] = allQuestions.get(key);
      });
    }
  }

  /**
   * Helper that we call whenever we drag and drop
   * @param target The area that we are dragging and dropping in
   */
  function questionMoveHelper(target: string) {
    // for each element within our target list, set its current section to be this list
    for (let index = 0; index < allQuestions.get(target).length; index++) {
      var old: string = allQuestions.get(target)[index].section;
      if (old != target) {
        allQuestions.get(target)[index].section = target;
      }
    }
  }

  // These functions are called when their specific region has a drag and drop event
  // We update the list that we are modifying with set and =, and also call our helper
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

  /**
   * add callback to insert the created object into the appropriate list. This just gets fed into adder
   * @param label The question label
   * @param type The type of question (see Adder.svelte or Question.svelte for number values)
   * @param section The section we are adding to (e.g. "prematch")
   * @param options Any options attached to the question
   */
  function add(
    label: string,
    type: number,
    section: string,
    options: string[]
  ) {
    // set and = to forcibly update the section with our new object
    allQuestions.set(section, [
      ...allQuestions.get(section),
      {
        label: label,
        type: type,
        section: section,
        options: options,
        deletable: true, // allow all user creatred questions to be deleted
        id: id++, // increment our ID to keep the ID unique
      },
    ]);
    allQuestions[section] = allQuestions.get(section);
  }

  /**
   * invokes rust command to write our data to an output file
   * Will let you pick location, but not file name, bc that's hardcoded in the scouting app
   */
  async function saveFile() {
    // Open folder dialog, looks at desktop by default
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: await desktopDir(),
    });

    // if the user didn't pick a folder, exit
    if (selected === null) {
      return;
    } else {
      // Otherwise, stringify our map and write it to the file
      invoke("write_output", {
        path: selected,
        data: JSON.stringify(allQuestions),
      });
    }
  }
</script>

<main class="box">
  <div>
    <h2>Prematch</h2>
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
          id={question.id}
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
          id={question.id}
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
          id={question.id}
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
          id={question.id}
        />
      {/each}
    </div>
    <h2>Pit Scouting</h2>
    <div
      class="slot"
      use:dndzone={{ items: allQuestions.get("pit") }}
      on:consider={considerPrematch}
      on:finalize={finalizePrematch}
    >
      {#each allQuestions.get("pit") as question (question.id)}
        <Question
          label={question.label}
          options={question.options}
          type={question.type}
          section={question.section}
          onDelete={del}
          deletable={question.deletable}
          id={question.id}
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
