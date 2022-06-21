# GitLab Pipeline Viewer

Monitor your GitLab pipelines from your terminal.  
Useful for frequent pushes to a feature branch
or merge request.

## How to use
Run the binary in the working directory of your
poject on the terminal.  
Now work, checkout and push in a different window.  
_gpv_ will always show you your latest pipeline of your
checkouted (?) branch.


_Specs_:
- Lot of details
- No browsing
- Works on multiple GitLab instances
- Always latest pipeline
- Changes the branch when you do

## Demo output
```
                                                 ====   GitLab Pipeline Viewer   ====                                                  
                                       https://gitlab.com/julianbuettner/gitlab-pipeline-viewer                                        
                                         View your GitLab pipelines from within your terminal                                          
                                                                                                                                       
                                                   ====   Pipeline 569633322   ====                                                    
                            https://gitlab.com/julianbuettner/gitlab-pipeline-viewer/-/pipelines/569633322                             
                                              by Julian Büttner 23 minutes 14 seconds ago                                              
                                       add-gitlab-ci @ 7d4031ec0ef444a3a69c9b512a14f5c7f631c744                                        
                                                  ✅  passed in 12 minutes 56 seconds                                                  
                                                                                                                                       
                                                                                                                                       
                     =====   analysis   =====                                           =====   build   =====                       
                                                                                                                                    
                         ✅  cargo-check                                                   ✅  cargo-build                          
                5 minutes 36 seconds gitlab-runner                                7 minutes 19 seconds gitlab-runner                
                                                                                                                                    
                         ✅  cargo-format                                                                                           
                     28 seconds gitlab-runner                                                                                       

```

## Config
```yaml
---
# ~/.gitlab-pipeline-viewer.yaml

gitlab-tokens:
  gitlab.com: gl-abcdefghijk
  gitlab.mysite.com: gl-123456789

# Default remote is origin.
remote: origin

# When to refresh the dashboard.
# Default is 5 seconds,
# but I think GitLab can handle a lot more updates.
cooldown: 3

```

## Ideas for future features

- [ ] Coloring
- [ ] Realtime updates with websocket o.e.
- [ ] Save / display log of (failing) jobs
- [ ] Support pipelines other than GitLab
  - [ ] Make pipeline data structure generic
  - [ ] Write support for other pipelines
