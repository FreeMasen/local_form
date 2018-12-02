# Local Form
This application spins up a local server on port 8787, this server will
serve any static files that are located in a folder `assets`.
The assets folder needs to be in the same directory as the the application.

The applications expects that the html form's action attribute to be set to "/submit".

When a form is submitted to that route, it will pull the `application/x-www-form-urlencoded`
parameters out and write them to a file `form-entries.csv`. If it hasn't already created a csv,
it will use the key's of the parameters and sets them as the first csv row. There is no guaranteed
order but each row will always match the headers.

If the form has changed, it is important that you move or re-move the `form-entries.csv` file
if not you may lose some form data.

On the [releases](https://github.com/FreeMasen/local_form/releases) page you can find a binary
for Linux, MaxOS, and Windows. The most recent version will be at the top of the page, it is best
to use that.