# youtrack-with-gitlab-integration

Integration for youtrack with gitlab

### Integration steps

1. Gitlab:
    - Create web hook on **Merge request events** with **secret token** and chosen **url**
    - rr
2. Youtrack:
    - create ne workflow with content from the [file](workflow/index.js)

### Dev install
- cargo-watch


### Running prod
- `run --release --package Image-loading-API --bin web-app`

### Running dev server
- `cargo build`
- `systemfd --no-pid -s http::3000 -- cargo watch -x run`
- `RUSTFLAGS=-Awarnings systemfd --no-pid -s http::3000 -- cargo watch -x run`
- ` cargo build 2>&1 | rg -i --multiline "(^error.*\n.*)|(aborting)|(warnings)"`
- `cargo watch -x 'clippy'`


### What is done

- Youtrack sdk
    - [X] Issue reading
    - [X] Issue status changing
    - [ ] Comment adding with link to the gitlab
    - [ ] Endpoint on status changing
- Gitlab:
    - [ ] Checking of merge requests creation
    - [ ] Checking merging of tasks
    - [ ] Endpoint for webhooks events (pipeline finishing, merge, comments):
        - [ ] endpoint (check token, parse request, do some work on the youtrack)
        - [X] Deserialization for web hooks
- Common:
    - [ ] On Merging should check. If pipline not finished:
        - [ ] set label "Pipeline in progress"
        - [ ] remove label when pipeline will be done
    - [ ] If was added comment to the merge request, then task should be labeled with "review comments"
- Settings:
    - Events:
      - Gitlab:
        - [ ] on comment (in progress)
        - on pipeline:
            - [ ] started
            - [ ] success
            - [ ] failed
        - on merge request:
            - [ ] created
            - [ ] updated
            - [ ] conflict
            - [ ] merged
        
    - Operations:
        - [X] type
        - [X] custom-args
        - [X] args
        - [X] update
        - [X] filter
        - filter args:
            - [X] equals
            - [ ] has
    
### Unimplemented
- [ ] find_issues

### Config features:
- [ ] Make ChangeTasks parser
- [ ] Make FilterArgsEquals/FilterArgsHas comparable with args


