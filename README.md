# ssgithub
inspired from downgit
## tasks
- [ ] parse request path
  - [ ] get repository details
  - [ ] get branch 
  - [ ] get file/folder path
  - [ ] get file/folder name
- [ ] get repo info using github api
... plan further


Rate limits are
60/hr for unauthenticated requests
5000/hr for authenticated requests

example = https://api.github.com/repos/requestly/requestly-desktop-app/contents/src?ref=rename-requestly-package

for making sure that html files are downloaded and not served -- https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition




future improvements 
idk: also copy git history into the created zip (seems possible since api response lso gives the corresponding git link)