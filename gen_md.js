const fs = require('node:fs');
const filename = 'README.md';
let buf = '# Quizzes' + '\n';
for (const unit of fs.readdirSync('quiz')) {
    buf += '## Unit ' + unit.slice(1);
    const important = [];
    let lessons = '';
    for (const lesson of fs.readdirSync('quiz/' + unit)) {
        if (lesson.endsWith('md')) {
            const name = lesson.slice(0, -3);
            important.push('*[' + name + '](' + unit + '/' + name + '.md)*');
        } else {
            lessons += '* Lesson ' + lesson.slice(1) + '\n';
            for (const md of fs.readdirSync('quiz/' + unit + '/' + lesson)) {
                lessons += '\t* [' + md.replace(/_/g, ' ').slice(0, -3) + '](' + unit + '/' + lesson + '/' + md.replace(/\./g, '_') + ')' + '\n';
            }
        }

        for (const activity of fs.readdirSync('src').filter(x=>x.startsWith(unit + '_' + lesson))) {
            const name = (activity.match(/Activity_\w+\.java$/g) || [''])[0];
            console.log(name);
            if (name) {
                lessons += '\t* *[Coding Activity ';
                lessons += name.replace(/^Activity_|\.java$/g, '');
                lessons += '](src/'+activity+')*\n';
            }
            
        }

    }

    if (important.length) {
        buf += ' (' + important.join(', ') + ')';
    }
    buf += '\n';
    buf += lessons;
}

fs.writeFileSync('README.md', buf)