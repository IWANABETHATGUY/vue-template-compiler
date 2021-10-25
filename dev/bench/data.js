window.BENCHMARK_DATA = {
  "lastUpdate": 1635165468897,
  "repoUrl": "https://github.com/IWANABETHATGUY/vue-template-compiler",
  "entries": {
    "Benchmark @vue canonical compiler": [
      {
        "commit": {
          "author": {
            "email": "bot@renovateapp.com",
            "name": "Renovate Bot",
            "username": "renovate-bot"
          },
          "committer": {
            "email": "2883231+HerringtonDarkholme@users.noreply.github.com",
            "name": "Herrington Darkholme",
            "username": "HerringtonDarkholme"
          },
          "distinct": false,
          "id": "9e910157b1756bb9e286b9650ede62c0040d6bce",
          "message": "Update dependency vite to v2.6.11",
          "timestamp": "2021-10-25T08:00:09-04:00",
          "tree_id": "77eb1facaa8831b28d9a3e2a0028ce90f71a17a0",
          "url": "https://github.com/IWANABETHATGUY/vue-template-compiler/commit/9e910157b1756bb9e286b9650ede62c0040d6bce"
        },
        "date": 1635165383112,
        "tool": "benchmarkjs",
        "benches": [
          {
            "name": "Attribute.vue",
            "value": 31217,
            "range": "±2.33%",
            "unit": "ops/sec",
            "extra": "85 samples"
          },
          {
            "name": "Counter.vue",
            "value": 50472,
            "range": "±0.92%",
            "unit": "ops/sec",
            "extra": "90 samples"
          },
          {
            "name": "ElasticHeader.vue",
            "value": 3446,
            "range": "±9.81%",
            "unit": "ops/sec",
            "extra": "84 samples"
          },
          {
            "name": "GithubCommit.vue",
            "value": 4945,
            "range": "±8.06%",
            "unit": "ops/sec",
            "extra": "85 samples"
          },
          {
            "name": "ModalComponent.vue",
            "value": 9111,
            "range": "±2.31%",
            "unit": "ops/sec",
            "extra": "87 samples"
          },
          {
            "name": "TodoApp.vue",
            "value": 1871,
            "range": "±1.03%",
            "unit": "ops/sec",
            "extra": "87 samples"
          },
          {
            "name": "TreeView.vue",
            "value": 4476,
            "range": "±1.60%",
            "unit": "ops/sec",
            "extra": "82 samples"
          },
          {
            "name": "TwoWayBinding.vue",
            "value": 23193,
            "range": "±0.86%",
            "unit": "ops/sec",
            "extra": "90 samples"
          },
          {
            "name": "UserInput.vue",
            "value": 21247,
            "range": "±0.74%",
            "unit": "ops/sec",
            "extra": "86 samples"
          },
          {
            "name": "VFor.vue",
            "value": 24099,
            "range": "±0.69%",
            "unit": "ops/sec",
            "extra": "91 samples"
          },
          {
            "name": "VIf.vue",
            "value": 33384,
            "range": "±0.70%",
            "unit": "ops/sec",
            "extra": "91 samples"
          }
        ]
      }
    ],
    "Benchmark rusty vue compiler": [
      {
        "commit": {
          "author": {
            "email": "bot@renovateapp.com",
            "name": "Renovate Bot",
            "username": "renovate-bot"
          },
          "committer": {
            "email": "2883231+HerringtonDarkholme@users.noreply.github.com",
            "name": "Herrington Darkholme",
            "username": "HerringtonDarkholme"
          },
          "distinct": false,
          "id": "cc005a01bf26ddb39bee9b4081ebc87bc16bec7c",
          "message": "Update dependency vue-tsc to v0.28.8",
          "timestamp": "2021-10-24T00:29:02-04:00",
          "tree_id": "85fe57c5dea0f3e3293cd72d18e828b7ef67b2a4",
          "url": "https://github.com/IWANABETHATGUY/vue-template-compiler/commit/cc005a01bf26ddb39bee9b4081ebc87bc16bec7c"
        },
        "date": 1635165467923,
        "tool": "cargo",
        "benches": [
          {
            "name": "compile/Attribute",
            "value": 5695,
            "range": "± 191",
            "unit": "ns/iter"
          },
          {
            "name": "compile/Counter",
            "value": 3958,
            "range": "± 163",
            "unit": "ns/iter"
          },
          {
            "name": "compile/ElTable",
            "value": 241997,
            "range": "± 7041",
            "unit": "ns/iter"
          },
          {
            "name": "compile/ElasticHeader",
            "value": 46176,
            "range": "± 2715",
            "unit": "ns/iter"
          },
          {
            "name": "compile/GithubCommit",
            "value": 37579,
            "range": "± 1277",
            "unit": "ns/iter"
          },
          {
            "name": "compile/ModalComponent",
            "value": 23674,
            "range": "± 927",
            "unit": "ns/iter"
          },
          {
            "name": "compile/TodoApp",
            "value": 113583,
            "range": "± 4881",
            "unit": "ns/iter"
          },
          {
            "name": "compile/TreeView",
            "value": 23334,
            "range": "± 868",
            "unit": "ns/iter"
          },
          {
            "name": "compile/TwoWayBinding",
            "value": 6966,
            "range": "± 359",
            "unit": "ns/iter"
          },
          {
            "name": "compile/UserInput",
            "value": 7088,
            "range": "± 319",
            "unit": "ns/iter"
          },
          {
            "name": "compile/VFor",
            "value": 6986,
            "range": "± 242",
            "unit": "ns/iter"
          },
          {
            "name": "compile/VIf",
            "value": 5769,
            "range": "± 193",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}